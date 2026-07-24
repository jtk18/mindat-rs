//! HTTP client for the Mindat API.

use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use reqwest::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT};
use std::time::Duration;
use url::Url;

use crate::error::{MindatError, Result};
use crate::models::*;

/// Default base URL for the Mindat API (v1).
/// Note: Must end with a slash for proper URL joining.
pub const DEFAULT_BASE_URL: &str = "https://api.mindat.org/v1/";

/// User-Agent string for API requests.
/// Using a browser-like User-Agent to avoid Cloudflare blocks.
const USER_AGENT_STRING: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

/// Create a configured HTTP client with proper timeouts and settings.
fn create_http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(5)
        .build()
        .expect("Failed to create HTTP client")
}

/// Join a list of i32 IDs into the comma-separated form Mindat's `*_in` filters expect.
fn join_ids(ids: &[i32]) -> String {
    ids.iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

/// Join a list of i64 IDs into the comma-separated form Mindat's `*_in` filters expect.
fn join_ids64(ids: &[i64]) -> String {
    ids.iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

/// Replace raw ASCII control characters (U+0000–U+001F) with spaces so a response
/// containing unescaped control characters still parses.
///
/// The Mindat API sometimes returns a literal newline inside a free-text field
/// (e.g. a locality's `description_short`), which is invalid JSON that
/// `serde_json` rejects. Control bytes are never structurally required in JSON,
/// and escaped sequences like `\n` are ordinary ASCII (`\` + `n`) and are left
/// untouched. Borrows the input unchanged when there is nothing to fix.
fn sanitize_control_chars(s: &str) -> std::borrow::Cow<'_, str> {
    if s.bytes().any(|b| b < 0x20) {
        std::borrow::Cow::Owned(
            s.chars()
                .map(|c| if (c as u32) < 0x20 { ' ' } else { c })
                .collect(),
        )
    } else {
        std::borrow::Cow::Borrowed(s)
    }
}

/// Parse a JSON value that may be a number or a numeric string into a positive i32
/// (Mindat sometimes returns IDs as strings, e.g. from `/geoloc-point/`).
fn json_id(v: &serde_json::Value) -> Option<i32> {
    let n = match v {
        serde_json::Value::Number(n) => n.as_i64(),
        serde_json::Value::String(s) => s.trim().parse::<i64>().ok(),
        _ => None,
    }?;
    (n > 0 && n <= i32::MAX as i64).then_some(n as i32)
}

/// Maximum number of concurrent geo-probe requests issued by the radius helpers.
const GEO_GRID_CONCURRENCY: usize = 12;

/// A hexagonal grid of probe points `(lat, lon, per-point distance km)` covering a
/// radius. Mindat's geo endpoints return only ~10 localities per point, so real
/// coverage needs a dense grid with a small per-point distance (a coarse grid
/// clusters near each probe and misses sites). Spacing scales with the radius and
/// each probe's distance equals the spacing so the cells overlap.
fn hex_grid(lat: f64, lon: f64, radius_km: f64) -> Vec<(f64, f64, f64)> {
    let s = (radius_km / 4.5).clamp(35.0, 90.0);
    let coslat = lat.to_radians().cos().abs().max(0.05);
    let n = (radius_km / s).ceil() as i64 + 1;
    let mut pts = Vec::new();
    for j in -n..=n {
        for i in -n..=n {
            let x = s * (i as f64 + 0.5 * (j.rem_euclid(2) as f64));
            let y = s * (j as f64) * 0.866;
            if (x * x + y * y).sqrt() <= radius_km {
                pts.push((lat + y / 111.0, lon + x / (111.0 * coslat), s));
            }
        }
    }
    if pts.is_empty() {
        pts.push((lat, lon, radius_km.max(1.0)));
    }
    pts.truncate(96);
    pts
}

/// Percent-encode a value that will be interpolated into a URL path segment,
/// so caller-supplied strings (e.g. ISBN/DDC/LCC codes) can't inject `../`,
/// extra path segments, or `?`/`#` into the request target.
fn encode_segment(s: &str) -> String {
    utf8_percent_encode(s, NON_ALPHANUMERIC).to_string()
}

/// Serialize a single enum/scalar value to the string form used in a query
/// parameter (the API expects e.g. `Hexagonal`, `APPROVED`, `0`).
fn param_value<T: serde::Serialize>(v: &T) -> Option<String> {
    match serde_json::to_value(v).ok()? {
        serde_json::Value::String(s) => Some(s),
        serde_json::Value::Bool(b) => Some(b.to_string()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        serde_json::Value::Null => None,
        other => Some(other.to_string()),
    }
}

/// Push an optional string parameter.
fn push_opt(params: &mut Vec<(&'static str, String)>, key: &'static str, val: Option<String>) {
    if let Some(v) = val {
        params.push((key, v));
    }
}

/// Push a `Display` scalar (numbers, bools) if present.
fn push_scalar<T: std::fmt::Display>(
    params: &mut Vec<(&'static str, String)>,
    key: &'static str,
    val: Option<T>,
) {
    if let Some(v) = val {
        params.push((key, v.to_string()));
    }
}

/// Push a multi-choice enum filter as one repeated parameter per value
/// (the API uses `?crystal_system=Hexagonal&crystal_system=Trigonal`).
fn push_multi<T: serde::Serialize>(
    params: &mut Vec<(&'static str, String)>,
    key: &'static str,
    vals: &Option<Vec<T>>,
) {
    if let Some(list) = vals {
        for item in list {
            if let Some(s) = param_value(item) {
                params.push((key, s));
            }
        }
    }
}

/// Push a single-choice enum filter if present.
fn push_enum<T: serde::Serialize>(
    params: &mut Vec<(&'static str, String)>,
    key: &'static str,
    val: &Option<T>,
) {
    if let Some(item) = val {
        if let Some(s) = param_value(item) {
            params.push((key, s));
        }
    }
}

/// Client for interacting with the Mindat API.
#[derive(Clone)]
pub struct MindatClient {
    http: Client,
    base_url: Url,
    token: Option<String>,
}

// Manual `Debug` so the API token is never printed (e.g. via `tracing`/logs).
impl std::fmt::Debug for MindatClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MindatClient")
            .field("base_url", &self.base_url.as_str())
            .field(
                "token",
                &self.token.as_ref().map(|_| "***REDACTED***"),
            )
            .finish()
    }
}

impl MindatClient {
    /// Create a new client with the given API token.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mindat_rs::MindatClient;
    ///
    /// let client = MindatClient::new("your-api-token");
    /// ```
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            http: create_http_client(),
            base_url: Url::parse(DEFAULT_BASE_URL).unwrap(),
            token: Some(token.into()),
        }
    }

    /// Create a new client without authentication.
    /// Some endpoints (like minerals_ima) work without authentication.
    pub fn anonymous() -> Self {
        Self {
            http: create_http_client(),
            base_url: Url::parse(DEFAULT_BASE_URL).unwrap(),
            token: None,
        }
    }

    /// Create a new client builder for more configuration options.
    pub fn builder() -> MindatClientBuilder {
        MindatClientBuilder::new()
    }

    /// Set the API token.
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = Some(token.into());
    }

    /// Get the base URL.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Build request headers.
    fn headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        // Always include User-Agent and Accept to avoid Cloudflare blocks
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STRING));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        if let Some(ref token) = self.token {
            let auth_value = format!("Token {}", token);
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&auth_value).map_err(|_| {
                    MindatError::InvalidParameter("Invalid token format".to_string())
                })?,
            );
        }
        Ok(headers)
    }

    /// Make a GET request to the API.
    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        // Strip leading slash to ensure proper URL joining with base URL
        let path = path.strip_prefix('/').unwrap_or(path);
        let url = self.base_url.join(path)?;
        let response = self.http.get(url).headers(self.headers()?).send().await?;

        self.handle_response(response).await
    }

    /// Make a GET request with query parameters.
    async fn get_with_query<T, Q>(&self, path: &str, query: &Q) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        Q: serde::Serialize,
    {
        // Strip leading slash to ensure proper URL joining with base URL
        let path = path.strip_prefix('/').unwrap_or(path);
        let url = self.base_url.join(path)?;
        let response = self
            .http
            .get(url)
            .headers(self.headers()?)
            .query(query)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a POST request with a JSON body.
    async fn post_json<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let path = path.strip_prefix('/').unwrap_or(path);
        let url = self.base_url.join(path)?;
        let response = self
            .http
            .post(url)
            .headers(self.headers()?)
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Handle API response.
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            let text = response.text().await?;
            // The API occasionally emits unescaped control characters (e.g. raw
            // newlines in free-text fields); scrub them so parsing doesn't fail.
            serde_json::from_str(&sanitize_control_chars(&text)).map_err(MindatError::from)
        } else {
            let status_code = status.as_u16();
            let mut message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Cap the upstream body embedded in the error so a large or
            // sensitive response body can't bloat logs or leak in full.
            const MAX_ERR_BODY: usize = 2048;
            if message.len() > MAX_ERR_BODY {
                let mut end = MAX_ERR_BODY;
                while !message.is_char_boundary(end) {
                    end -= 1;
                }
                message.truncate(end);
                message.push_str("… (truncated)");
            }

            match status_code {
                401 => Err(MindatError::AuthenticationRequired),
                404 => Err(MindatError::NotFound(message)),
                429 => Err(MindatError::RateLimited),
                _ => Err(MindatError::Api {
                    status: status_code,
                    message,
                }),
            }
        }
    }

    // ==================== Geomaterials ====================

    /// List geomaterials with optional filters.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> mindat_rs::Result<()> {
    /// use mindat_rs::{MindatClient, GeomaterialsQuery};
    ///
    /// let client = MindatClient::new("your-token");
    ///
    /// // Get IMA-approved minerals containing copper
    /// let query = GeomaterialsQuery::new()
    ///     .ima_approved(true)
    ///     .with_elements("Cu")
    ///     .page_size(50);
    ///
    /// let minerals = client.geomaterials(query).await?;
    /// for mineral in minerals.results {
    ///     println!("{}: {:?}", mineral.id, mineral.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn geomaterials(
        &self,
        query: GeomaterialsQuery,
    ) -> Result<PaginatedResponse<Geomaterial>> {
        // Built as an ordered list of key/value pairs (rather than a serde
        // struct) so that multi-choice filters can be sent as repeated
        // parameters, e.g. `?crystal_system=Hexagonal&crystal_system=Trigonal`,
        // which is how the Mindat API expects `explode=true` array filters.
        let mut p: Vec<(&'static str, String)> = Vec::new();

        push_opt(&mut p, "name", query.name);
        push_opt(&mut p, "q", query.q);
        push_scalar(&mut p, "ima", query.ima);
        push_opt(&mut p, "el_inc", query.elements_inc);
        push_opt(&mut p, "el_exc", query.elements_exc);
        push_opt(&mut p, "el_essential", query.el_essential);
        // Multi-choice enum filters (repeated params).
        push_multi(&mut p, "ima_status", &query.ima_status);
        push_multi(&mut p, "ima_notes", &query.ima_notes);
        push_multi(&mut p, "entrytype", &query.entrytype);
        push_multi(&mut p, "crystal_system", &query.crystal_system);
        push_multi(&mut p, "cleavagetype", &query.cleavagetype);
        push_multi(&mut p, "fracturetype", &query.fracturetype);
        push_multi(&mut p, "lustretype", &query.lustretype);
        push_multi(&mut p, "diapheny", &query.diapheny);
        push_multi(&mut p, "tenacity", &query.tenacity);
        push_enum(&mut p, "opticaltype", &query.opticaltype);
        push_enum(&mut p, "opticalsign", &query.opticalsign);
        push_opt(&mut p, "colour", query.colour);
        push_opt(&mut p, "streak", query.streak);
        push_scalar(&mut p, "hardness_min", query.hardness_min);
        push_scalar(&mut p, "hardness_max", query.hardness_max);
        push_scalar(&mut p, "density_min", query.density_min);
        push_scalar(&mut p, "density_max", query.density_max);
        push_scalar(&mut p, "ri_min", query.ri_min);
        push_scalar(&mut p, "ri_max", query.ri_max);
        push_opt(&mut p, "bi_min", query.bi_min);
        push_opt(&mut p, "bi_max", query.bi_max);
        push_opt(&mut p, "optical2v_min", query.optical2v_min);
        push_opt(&mut p, "optical2v_max", query.optical2v_max);
        push_scalar(&mut p, "varietyof", query.varietyof);
        push_scalar(&mut p, "synid", query.synid);
        push_scalar(&mut p, "polytypeof", query.polytypeof);
        push_scalar(&mut p, "groupid", query.groupid);
        push_opt(&mut p, "id_in", query.id_in.as_deref().map(join_ids));
        push_scalar(&mut p, "id_min", query.id_min);
        push_scalar(&mut p, "id_max", query.id_max);
        push_scalar(&mut p, "non_utf", query.non_utf);
        push_opt(&mut p, "meteoritical_code", query.meteoritical_code);
        push_scalar(
            &mut p,
            "meteoritical_code_exists",
            query.meteoritical_code_exists,
        );
        push_opt(&mut p, "updated_at", query.updated_at);
        push_opt(&mut p, "fields", query.fields);
        push_opt(&mut p, "omit", query.omit);
        push_opt(&mut p, "expand", query.expand.map(|e| e.join(",")));
        push_opt(&mut p, "ordering", query.ordering.map(|o| o.to_string()));
        push_scalar(&mut p, "page", query.page);
        push_scalar(&mut p, "page-size", query.page_size);

        self.get_with_query("/geomaterials/", &p).await
    }

    /// Get a specific geomaterial by ID.
    pub async fn geomaterial(&self, id: i32) -> Result<Geomaterial> {
        self.get(&format!("/geomaterials/{}/", id)).await
    }

    /// Get varieties of a specific geomaterial.
    pub async fn geomaterial_varieties(&self, id: i32) -> Result<Geomaterial> {
        self.get(&format!("/geomaterials/{}/varieties/", id)).await
    }

    /// Search for geomaterials.
    pub async fn geomaterials_search(
        &self,
        q: &str,
        size: Option<i32>,
    ) -> Result<Vec<serde_json::Value>> {
        #[derive(serde::Serialize)]
        struct Query<'a> {
            q: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            size: Option<i32>,
        }
        self.get_with_query("/geomaterials-search/", &Query { q, size })
            .await
    }

    /// Get the geomaterials field dictionary (distinct values for a field).
    pub async fn geomaterials_dict(&self, field: Option<&str>) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            field: Option<&'a str>,
        }
        self.get_with_query("/geomaterials/dict/", &Query { field })
            .await
    }

    // ==================== Localities ====================

    /// List localities with optional filters.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> mindat_rs::Result<()> {
    /// use mindat_rs::{MindatClient, LocalitiesQuery};
    ///
    /// let client = MindatClient::new("your-token");
    ///
    /// // Get localities in Brazil
    /// let query = LocalitiesQuery::new().country("Brazil");
    /// let localities = client.localities(query).await?;
    /// for loc in localities.results {
    ///     println!("{}: {:?}", loc.id, loc.txt);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn localities(
        &self,
        query: LocalitiesQuery,
    ) -> Result<PaginatedResponse<Locality>> {
        // Note: `/localities/` uses page-number pagination (`page` / `page-size`);
        // the older cursor pagination was removed from the Mindat API.
        // It also keeps the `elements_inc` / `elements_exc` filter names
        // (unlike geomaterials, which moved to `el_inc` / `el_exc`).
        #[derive(serde::Serialize)]
        struct QueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            country: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            txt: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            elements_inc: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            elements_exc: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id_in: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            level_gte: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            level_lte: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sublocs_gte: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sublocs_lte: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            non_hierarchial: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            revtxtd_istartswith: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fields: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            omit: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            expand: Option<String>,
            #[serde(rename = "page-size", skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
        }

        let params = QueryParams {
            country: query.country,
            txt: query.txt,
            description: query.description,
            elements_inc: query.elements_inc,
            elements_exc: query.elements_exc,
            id_in: query.id_in.as_deref().map(join_ids),
            level_gte: query.level_gte,
            level_lte: query.level_lte,
            sublocs_gte: query.sublocs_gte,
            sublocs_lte: query.sublocs_lte,
            non_hierarchial: query.non_hierarchial,
            revtxtd_istartswith: query.revtxtd_istartswith,
            updated_at: query.updated_at,
            fields: query.fields,
            omit: query.omit,
            expand: query.expand.map(|e| e.join(",")),
            page_size: query.page_size,
            page: query.page,
        };

        self.get_with_query("/localities/", &params).await
    }

    /// Get a specific locality by ID.
    pub async fn locality(&self, id: i32) -> Result<Locality> {
        self.get(&format!("/localities/{}/", id)).await
    }

    // ==================== Locality Metadata ====================

    /// List locality ages.
    pub async fn locality_ages(&self, page: Option<i32>) -> Result<PaginatedResponse<LocalityAge>> {
        #[derive(serde::Serialize)]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
        }
        self.get_with_query("/locality-age/", &Query { page }).await
    }

    /// Get a specific locality age by ID.
    pub async fn locality_age(&self, age_id: i32) -> Result<LocalityAge> {
        self.get(&format!("/locality-age/{}/", age_id)).await
    }

    /// List locality statuses.
    pub async fn locality_statuses(
        &self,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<LocalityStatus>> {
        #[derive(serde::Serialize)]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
        }
        self.get_with_query("/locality-status/", &Query { page })
            .await
    }

    /// Get a specific locality status by ID.
    pub async fn locality_status(&self, ls_id: i32) -> Result<LocalityStatus> {
        self.get(&format!("/locality-status/{}/", ls_id)).await
    }

    /// List locality types.
    pub async fn locality_types(
        &self,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<LocalityType>> {
        #[derive(serde::Serialize)]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
        }
        self.get_with_query("/locality-type/", &Query { page })
            .await
    }

    /// Get a specific locality type by ID.
    pub async fn locality_type(&self, lt_id: i32) -> Result<LocalityType> {
        self.get(&format!("/locality-type/{}/", lt_id)).await
    }

    /// List locality name translations.
    pub async fn locality_translations(
        &self,
        query: LocalityTranslationsQuery,
    ) -> Result<PaginatedResponse<LocalityTranslation>> {
        #[derive(serde::Serialize)]
        struct QueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            lt_loc: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lt_iso: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lt_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lt_important: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lt_datetime_gte: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ordering: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(rename = "page-size", skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }
        let params = QueryParams {
            lt_loc: query.lt_loc,
            lt_iso: query.lt_iso,
            lt_text: query.lt_text,
            lt_important: query.lt_important,
            lt_datetime_gte: query.lt_datetime_gte,
            ordering: query.ordering,
            page: query.page,
            page_size: query.page_size,
        };
        self.get_with_query("/locality-translations/", &params)
            .await
    }

    /// Get a specific locality translation by ID.
    pub async fn locality_translation(&self, lt_id: i32) -> Result<LocalityTranslation> {
        self.get(&format!("/locality-translations/{}/", lt_id))
            .await
    }

    // ==================== IMA Minerals ====================

    /// List IMA-approved minerals.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> mindat_rs::Result<()> {
    /// use mindat_rs::{MindatClient, ImaMineralsQuery};
    ///
    /// let client = MindatClient::anonymous(); // No auth required
    /// let query = ImaMineralsQuery::new().page_size(100);
    /// let minerals = client.minerals_ima(query).await?;
    /// for mineral in minerals.results {
    ///     println!("{}: {:?}", mineral.id, mineral.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn minerals_ima(
        &self,
        query: ImaMineralsQuery,
    ) -> Result<PaginatedResponse<ImaMaterial>> {
        #[derive(serde::Serialize)]
        struct QueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            q: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ima: Option<i32>,
            #[serde(rename = "el_inc", skip_serializing_if = "Option::is_none")]
            elements_inc: Option<String>,
            #[serde(rename = "el_exc", skip_serializing_if = "Option::is_none")]
            elements_exc: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            el_essential: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            colour: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            streak: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            hardness_min: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            hardness_max: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            density_min: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            density_max: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ri_min: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ri_max: Option<f32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            varietyof: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            synid: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            polytypeof: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            groupid: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            meteoritical_code: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            meteoritical_code_exists: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            non_utf: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id_in: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id_min: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id_max: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fields: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            omit: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            expand: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(rename = "page-size", skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }

        let params = QueryParams {
            q: query.q,
            name: query.name,
            ima: query.ima,
            elements_inc: query.elements_inc,
            elements_exc: query.elements_exc,
            el_essential: query.el_essential,
            colour: query.colour,
            streak: query.streak,
            hardness_min: query.hardness_min,
            hardness_max: query.hardness_max,
            density_min: query.density_min,
            density_max: query.density_max,
            ri_min: query.ri_min,
            ri_max: query.ri_max,
            varietyof: query.varietyof,
            synid: query.synid,
            polytypeof: query.polytypeof,
            groupid: query.groupid,
            meteoritical_code: query.meteoritical_code,
            meteoritical_code_exists: query.meteoritical_code_exists,
            non_utf: query.non_utf,
            id_in: query.id_in.as_deref().map(join_ids),
            id_min: query.id_min,
            id_max: query.id_max,
            updated_at: query.updated_at,
            fields: query.fields,
            omit: query.omit,
            expand: query.expand.map(|e| e.join(",")),
            page: query.page,
            page_size: query.page_size,
        };

        self.get_with_query("/minerals-ima/", &params).await
    }

    /// Get a specific IMA mineral by ID.
    pub async fn mineral_ima(&self, id: i32) -> Result<Geomaterial> {
        self.get(&format!("/minerals-ima/{}/", id)).await
    }

    // ==================== Classification Systems ====================

    // Note: the `/dana-8/` and `/nickel-strunz-10/` base list endpoints appear
    // in the published OpenAPI schema but return 404 on the live server, so
    // they are intentionally not exposed. Use the group/class/subclass/family
    // and by-id methods below instead.

    /// Get Dana 8th edition classification groups.
    pub async fn dana8_groups(&self) -> Result<serde_json::Value> {
        self.get("/dana-8/groups/").await
    }

    /// Get Dana 8th edition classification subgroups.
    pub async fn dana8_subgroups(&self) -> Result<serde_json::Value> {
        self.get("/dana-8/subgroups/").await
    }

    /// Get a specific Dana 8th edition classification.
    pub async fn dana8(&self, id: i32) -> Result<serde_json::Value> {
        self.get(&format!("/dana-8/{}/", id)).await
    }

    /// Get Nickel-Strunz 10th edition classification classes.
    pub async fn strunz10_classes(&self) -> Result<serde_json::Value> {
        self.get("/nickel-strunz-10/classes/").await
    }

    /// Get Nickel-Strunz 10th edition classification subclasses.
    pub async fn strunz10_subclasses(&self) -> Result<serde_json::Value> {
        self.get("/nickel-strunz-10/subclasses/").await
    }

    /// Get Nickel-Strunz 10th edition classification families.
    pub async fn strunz10_families(&self) -> Result<serde_json::Value> {
        self.get("/nickel-strunz-10/families/").await
    }

    /// Get a specific Nickel-Strunz 10th edition classification.
    pub async fn strunz10(&self, id: i32) -> Result<serde_json::Value> {
        self.get(&format!("/nickel-strunz-10/{}/", id)).await
    }

    /// Helper: fetch a simple page-based list.
    async fn list_page<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<PaginatedResponse<T>> {
        #[derive(serde::Serialize)]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(rename = "page-size", skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }
        self.get_with_query(path, &Query { page, page_size }).await
    }

    // ==================== References ====================

    /// List bibliographic references.
    pub async fn references(
        &self,
        query: ReferencesQuery,
    ) -> Result<PaginatedResponse<Reference>> {
        #[derive(serde::Serialize)]
        struct QueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            q: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ref_type: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ref_title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ref_journal: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ref_publisher: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ref_doi: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ref_isbn: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id_in: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id_range_min: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id_range_max: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ordering: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fields: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            omit: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(rename = "page-size", skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }
        let params = QueryParams {
            q: query.q,
            ref_type: query.ref_type,
            ref_title: query.ref_title,
            ref_journal: query.ref_journal,
            ref_publisher: query.ref_publisher,
            ref_doi: query.ref_doi,
            ref_isbn: query.ref_isbn,
            id_in: query.id_in.as_deref().map(join_ids64),
            id_range_min: query.id_range_min,
            id_range_max: query.id_range_max,
            ordering: query.ordering,
            fields: query.fields,
            omit: query.omit,
            page: query.page,
            page_size: query.page_size,
        };
        self.get_with_query("/references/", &params).await
    }

    /// Get a specific reference by ID.
    pub async fn reference(&self, ref_id: i64) -> Result<Reference> {
        self.get(&format!("/references/{}/", ref_id)).await
    }

    /// Get the references field dictionary (distinct values for a field).
    pub async fn references_dict(&self, field: Option<&str>) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            field: Option<&'a str>,
        }
        self.get_with_query("/references/dict/", &Query { field })
            .await
    }

    /// List reference-author links, optionally filtered by reference ID.
    pub async fn reference_authors(
        &self,
        ref_id: Option<i64>,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<ReferenceAuthor>> {
        #[derive(serde::Serialize)]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            ra_ref_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
        }
        self.get_with_query("/reference-authors/", &Query { ra_ref_id: ref_id, page })
            .await
    }

    /// Get a specific reference-author link by ID.
    pub async fn reference_author(&self, ra_id: i64) -> Result<ReferenceAuthor> {
        self.get(&format!("/reference-authors/{}/", ra_id)).await
    }

    /// List de-duplicated reference author names.
    pub async fn reference_authors_unique(
        &self,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<ReferenceAuthorUnique>> {
        self.list_page("/reference-authors-unique/", page, None).await
    }

    /// Get a de-duplicated reference author by ID.
    pub async fn reference_author_unique(&self, rau_id: i64) -> Result<ReferenceAuthorUnique> {
        self.get(&format!("/reference-authors-unique/{}/", rau_id))
            .await
    }

    /// List reference citations.
    pub async fn reference_citations(
        &self,
        query: ReferenceCitationsQuery,
    ) -> Result<PaginatedResponse<ReferenceCitation>> {
        #[derive(serde::Serialize)]
        struct QueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            rc_min: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            rc_loc: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            rc_photo: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            rc_article: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            rc_ref_id: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            rc_entryid: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            rc_type: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ordering: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(rename = "page-size", skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }
        let params = QueryParams {
            rc_min: query.rc_min,
            rc_loc: query.rc_loc,
            rc_photo: query.rc_photo,
            rc_article: query.rc_article,
            rc_ref_id: query.rc_ref_id,
            rc_entryid: query.rc_entryid,
            rc_type: query.rc_type,
            ordering: query.ordering,
            page: query.page,
            page_size: query.page_size,
        };
        self.get_with_query("/reference-citations/", &params).await
    }

    /// Get a specific reference citation by ID.
    pub async fn reference_citation(&self, rc_id: i64) -> Result<ReferenceCitation> {
        self.get(&format!("/reference-citations/{}/", rc_id)).await
    }

    /// Get the reference-citations field dictionary (distinct values for a field).
    pub async fn reference_citations_dict(&self, field: Option<&str>) -> Result<serde_json::Value> {
        #[derive(serde::Serialize)]
        struct Query<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            field: Option<&'a str>,
        }
        self.get_with_query("/reference-citations/dict/", &Query { field })
            .await
    }

    /// List reference types.
    pub async fn reference_types(
        &self,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<ReferenceType>> {
        self.list_page("/reference-types/", page, None).await
    }

    /// Get a specific reference type by ID.
    pub async fn reference_type(&self, rt_id: i64) -> Result<ReferenceType> {
        self.get(&format!("/reference-types/{}/", rt_id)).await
    }

    /// List reference languages.
    pub async fn reference_languages(
        &self,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<ReferenceLanguage>> {
        self.list_page("/reference-languages/", page, None).await
    }

    /// Get a specific reference language by ID.
    pub async fn reference_language(&self, rl_id: i64) -> Result<ReferenceLanguage> {
        self.get(&format!("/reference-languages/{}/", rl_id)).await
    }

    /// List reference classification entries.
    pub async fn reference_classifications(
        &self,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<ReferenceClassify>> {
        self.list_page("/reference-classify/", page, None).await
    }

    /// Get a specific reference classification entry by ID.
    pub async fn reference_classification(&self, rc_id: i64) -> Result<ReferenceClassify> {
        self.get(&format!("/reference-classify/{}/", rc_id)).await
    }

    /// List Dewey Decimal Classification entries.
    pub async fn reference_ddc(
        &self,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<ReferenceDdc>> {
        self.list_page("/reference-ddc/", page, None).await
    }

    /// Get a specific DDC entry by code.
    pub async fn reference_ddc_code(&self, ddc_code: &str) -> Result<ReferenceDdc> {
        self.get(&format!("/reference-ddc/{}/", encode_segment(ddc_code)))
            .await
    }

    /// List Library of Congress Classification entries.
    pub async fn reference_lcc(
        &self,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<ReferenceLcc>> {
        self.list_page("/reference-lcc/", page, None).await
    }

    /// Get a specific LCC entry by code.
    pub async fn reference_lcc_code(&self, lcc_code: &str) -> Result<ReferenceLcc> {
        self.get(&format!("/reference-lcc/{}/", encode_segment(lcc_code)))
            .await
    }

    /// List ISBN entries.
    pub async fn reference_isbn(
        &self,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<ReferenceIsbn>> {
        self.list_page("/reference-isbn/", page, None).await
    }

    /// Get a specific ISBN entry.
    pub async fn reference_isbn_code(&self, ri_isbn: &str) -> Result<ReferenceIsbn> {
        self.get(&format!("/reference-isbn/{}/", encode_segment(ri_isbn)))
            .await
    }

    /// List supplementary reference-extra entries.
    pub async fn reference_extra(
        &self,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<ReferenceExtra>> {
        self.list_page("/reference-extra/", page, None).await
    }

    /// Get a specific reference-extra entry by ID.
    pub async fn reference_extra_item(&self, re_id: i64) -> Result<ReferenceExtra> {
        self.get(&format!("/reference-extra/{}/", re_id)).await
    }

    // ==================== Occurrences ====================

    /// List mineral-at-locality occurrences.
    pub async fn occurrences(
        &self,
        query: OccurrencesQuery,
    ) -> Result<PaginatedResponse<Occurrence>> {
        #[derive(serde::Serialize)]
        struct QueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            min: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            loc: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            colour: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            habit: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fluorescence: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            datemodify_after: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            datemodify_before: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ordering: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fields: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            omit: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(rename = "page-size", skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }
        let params = QueryParams {
            min: query.min,
            loc: query.loc,
            colour: query.colour,
            habit: query.habit,
            fluorescence: query.fluorescence,
            description: query.description,
            datemodify_after: query.datemodify_after,
            datemodify_before: query.datemodify_before,
            ordering: query.ordering,
            fields: query.fields,
            omit: query.omit,
            page: query.page,
            page_size: query.page_size,
        };
        self.get_with_query("/occurrences/", &params).await
    }

    /// Get a specific occurrence by ID.
    pub async fn occurrence(&self, id: i64) -> Result<Occurrence> {
        self.get(&format!("/occurrences/{}/", id)).await
    }

    /// List aggregated occurrence statistics.
    pub async fn occurrence_statistics(
        &self,
        query: OccurrenceStatisticsQuery,
    ) -> Result<PaginatedResponse<OccurrenceStatistics>> {
        #[derive(serde::Serialize)]
        struct QueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            lstm_min: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lstm_loc: Option<i64>,
            #[serde(rename = "lstm_photocount__gte", skip_serializing_if = "Option::is_none")]
            photocount_min: Option<i64>,
            #[serde(rename = "lstm_photocount__lte", skip_serializing_if = "Option::is_none")]
            photocount_max: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ordering: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(rename = "page-size", skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }
        let params = QueryParams {
            lstm_min: query.lstm_min,
            lstm_loc: query.lstm_loc,
            photocount_min: query.photocount_min,
            photocount_max: query.photocount_max,
            ordering: query.ordering,
            page: query.page,
            page_size: query.page_size,
        };
        self.get_with_query("/occurrences-statistics/", &params)
            .await
    }

    /// Get a specific occurrence-statistics record by ID.
    pub async fn occurrence_statistic(&self, lstm_id: i64) -> Result<OccurrenceStatistics> {
        self.get(&format!("/occurrences-statistics/{}/", lstm_id))
            .await
    }

    /// Find locality IDs containing (`inc`) and excluding (`exc`) minerals.
    ///
    /// `inc` (required by the API) and `exc` are comma-separated mineral-ID
    /// lists. Note: the upstream endpoint currently returns HTTP 500 when `exc`
    /// is omitted, so pass a non-empty `exc` for reliable results.
    pub async fn loc_by_min(&self, inc: &str, exc: Option<&str>) -> Result<Vec<i64>> {
        #[derive(serde::Serialize)]
        struct Query<'a> {
            inc: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            exc: Option<&'a str>,
        }
        self.get_with_query("/loc-by-min/", &Query { inc, exc }).await
    }

    // ==================== Crystallography ====================

    /// List crystal classes (point groups).
    pub async fn crystal_classes(
        &self,
        query: CrystalClassesQuery,
    ) -> Result<PaginatedResponse<CrystalClass>> {
        #[derive(serde::Serialize)]
        struct QueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            system: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            symbol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id_in: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(rename = "page-size", skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }
        let params = QueryParams {
            system: query.system,
            symbol: query.symbol,
            id_in: query.id_in.as_deref().map(join_ids),
            page: query.page,
            page_size: query.page_size,
        };
        self.get_with_query("/crystalclasses/", &params).await
    }

    /// Get a specific crystal class by ID.
    pub async fn crystal_class(&self, id: i32) -> Result<CrystalClass> {
        self.get(&format!("/crystalclasses/{}/", id)).await
    }

    /// List space groups.
    pub async fn space_groups(
        &self,
        query: SpaceGroupsQuery,
    ) -> Result<PaginatedResponse<SpaceGroup>> {
        #[derive(serde::Serialize)]
        struct QueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            cclass: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sgtext: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            id_in: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(rename = "page-size", skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }
        let params = QueryParams {
            cclass: query.cclass,
            sgtext: query.sgtext,
            id_in: query.id_in.as_deref().map(join_ids),
            page: query.page,
            page_size: query.page_size,
        };
        self.get_with_query("/spacegroups/", &params).await
    }

    /// Get a specific space group by ID.
    pub async fn space_group(&self, id: i32) -> Result<SpaceGroup> {
        self.get(&format!("/spacegroups/{}/", id)).await
    }

    /// List space group sets.
    pub async fn space_group_sets(
        &self,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<SpaceGroupSet>> {
        self.list_page("/spacegroupsets/", page, None).await
    }

    /// Get a specific space group set by ID.
    pub async fn space_group_set(&self, id: i32) -> Result<SpaceGroupSet> {
        self.get(&format!("/spacegroupsets/{}/", id)).await
    }

    // ==================== Relations ====================

    /// List mineral relations.
    pub async fn relations(
        &self,
        query: RelationsQuery,
    ) -> Result<PaginatedResponse<MineralRelation>> {
        #[derive(serde::Serialize)]
        struct QueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            q: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            min1: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            min2: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            rel: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(rename = "page-size", skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }
        let params = QueryParams {
            q: query.q,
            min1: query.min1,
            min2: query.min2,
            rel: query.rel,
            page: query.page,
            page_size: query.page_size,
        };
        self.get_with_query("/relations/", &params).await
    }

    /// Get a specific mineral relation by ID.
    pub async fn relation(&self, rid: i64) -> Result<MineralRelation> {
        self.get(&format!("/relations/{}/", rid)).await
    }

    // ==================== Geospatial ====================

    /// Query localities by a GeoJSON point (POST `/geoloc-point/`).
    pub async fn geoloc_point(&self, body: serde_json::Value) -> Result<serde_json::Value> {
        self.post_json("/geoloc-point/", &body).await
    }

    /// Query localities by a GeoJSON polygon (POST `/geoloc-poly/`).
    pub async fn geoloc_poly(&self, body: serde_json::Value) -> Result<serde_json::Value> {
        self.post_json("/geoloc-poly/", &body).await
    }

    /// Query mineral occurrences by a GeoJSON point (POST `/geomin-point/`).
    pub async fn geomin_point(&self, body: serde_json::Value) -> Result<serde_json::Value> {
        self.post_json("/geomin-point/", &body).await
    }

    /// Query mineral occurrences by a GeoJSON polygon (POST `/geomin-poly/`).
    pub async fn geomin_poly(&self, body: serde_json::Value) -> Result<serde_json::Value> {
        self.post_json("/geomin-poly/", &body).await
    }

    /// All locality IDs within `radius_km` of a point, de-duplicated.
    ///
    /// `/geoloc-point/` returns only ~10 localities per call regardless of the
    /// requested distance, so this probes a hexagonal grid of points across the
    /// radius concurrently and merges the results — surfacing sites well away from
    /// the exact center. Probe failures are skipped rather than failing the whole
    /// search.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> mindat_rs::Result<()> {
    /// let client = mindat_rs::MindatClient::new("your-token");
    /// let ids = client.localities_within(32.22, -110.97, 80.0).await?;
    /// println!("{} localities within ~80 km", ids.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn localities_within(
        &self,
        latitude: f64,
        longitude: f64,
        radius_km: f64,
    ) -> Result<Vec<i32>> {
        let sem = std::sync::Arc::new(tokio::sync::Semaphore::new(GEO_GRID_CONCURRENCY));
        let mut set = tokio::task::JoinSet::new();
        for (plat, plon, pdist) in hex_grid(latitude, longitude, radius_km) {
            let client = self.clone();
            let sem = std::sync::Arc::clone(&sem);
            set.spawn(async move {
                let _permit = sem.acquire_owned().await.ok();
                let body = serde_json::json!({
                    "point": { "lat": plat, "lon": plon },
                    "distance": format!("{}km", pdist.round().max(1.0) as i64),
                });
                match client.geoloc_point(body).await {
                    Ok(v) => v
                        .as_array()
                        .map(|a| a.iter().filter_map(json_id).collect::<Vec<_>>())
                        .unwrap_or_default(),
                    Err(_) => Vec::new(),
                }
            });
        }
        let mut ids: Vec<i32> = Vec::new();
        while let Some(res) = set.join_next().await {
            if let Ok(mut v) = res {
                ids.append(&mut v);
            }
        }
        ids.sort_unstable();
        ids.dedup();
        Ok(ids)
    }

    /// Minerals found near a point, mapped to the locality IDs that contain them:
    /// `mineral_id -> [locality_id, ...]`, de-duplicated.
    ///
    /// Like [`localities_within`](Self::localities_within), this works around the
    /// ~10-per-point cap on `/geomin-point/` by probing a hexagonal grid across
    /// the radius and merging. To find the localities near a point that contain a
    /// given species, look up its ID in the returned map.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> mindat_rs::Result<()> {
    /// let client = mindat_rs::MindatClient::new("your-token");
    /// let near = client.minerals_within(42.93, -76.57, 320.0).await?;
    /// let quartz_localities = near.get(&3337).cloned().unwrap_or_default();
    /// println!("{} nearby localities contain quartz", quartz_localities.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn minerals_within(
        &self,
        latitude: f64,
        longitude: f64,
        radius_km: f64,
    ) -> Result<std::collections::HashMap<i32, Vec<i32>>> {
        let sem = std::sync::Arc::new(tokio::sync::Semaphore::new(GEO_GRID_CONCURRENCY));
        let mut set = tokio::task::JoinSet::new();
        for (plat, plon, pdist) in hex_grid(latitude, longitude, radius_km) {
            let client = self.clone();
            let sem = std::sync::Arc::clone(&sem);
            set.spawn(async move {
                let _permit = sem.acquire_owned().await.ok();
                let body = serde_json::json!({
                    "point": { "lat": plat, "lon": plon },
                    "distance": format!("{}km", pdist.round().max(1.0) as i64),
                });
                let mut out: std::collections::HashMap<i32, Vec<i32>> =
                    std::collections::HashMap::new();
                // geomin-point -> { "<mineral_id>": [locality_id, ...], ... }
                if let Ok(serde_json::Value::Object(map)) = client.geomin_point(body).await {
                    for (k, v) in map {
                        if let (Ok(mid), Some(arr)) = (k.parse::<i32>(), v.as_array()) {
                            out.entry(mid)
                                .or_default()
                                .extend(arr.iter().filter_map(json_id));
                        }
                    }
                }
                out
            });
        }
        let mut merged: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
        while let Some(res) = set.join_next().await {
            if let Ok(map) = res {
                for (mid, locs) in map {
                    merged.entry(mid).or_default().extend(locs);
                }
            }
        }
        for locs in merged.values_mut() {
            locs.sort_unstable();
            locs.dedup();
        }
        Ok(merged)
    }

    // ==================== Exports ====================

    /// List the available bulk data exports (CSV/JSON download links).
    pub async fn exports(&self) -> Result<serde_json::Value> {
        self.get("/exports/").await
    }
}

/// Builder for MindatClient configuration.
#[derive(Clone)]
pub struct MindatClientBuilder {
    token: Option<String>,
    base_url: String,
    timeout: Option<std::time::Duration>,
}

// Manual `Debug` so the API token is never printed.
impl std::fmt::Debug for MindatClientBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MindatClientBuilder")
            .field("base_url", &self.base_url)
            .field("token", &self.token.as_ref().map(|_| "***REDACTED***"))
            .field("timeout", &self.timeout)
            .finish()
    }
}

impl MindatClientBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            token: None,
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: None,
        }
    }

    /// Set the API token.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Set a custom base URL.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set request timeout.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<MindatClient> {
        // Default to the same conservative timeouts as `MindatClient::new`,
        // so a builder-constructed client can't hang indefinitely on a slow
        // or unresponsive server. A caller-supplied timeout overrides the default.
        let client_builder = Client::builder()
            .timeout(self.timeout.unwrap_or_else(|| Duration::from_secs(30)))
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(5);

        let http = client_builder.build().map_err(MindatError::Request)?;

        let base_url = Url::parse(&self.base_url)?;

        Ok(MindatClient {
            http,
            base_url,
            token: self.token,
        })
    }
}

impl Default for MindatClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_replaces_raw_control_chars() {
        // A raw newline inside a JSON string is invalid; scrubbing makes it parse.
        let bad = "{\"a\": \"line1\nline2\ttab\"}";
        let cleaned = sanitize_control_chars(bad);
        assert!(!cleaned.bytes().any(|b| b < 0x20));
        let v: serde_json::Value = serde_json::from_str(&cleaned).unwrap();
        assert_eq!(v["a"], "line1 line2 tab");
    }

    #[test]
    fn sanitize_leaves_clean_json_borrowed_and_intact() {
        let good = "{\"a\":\"b\",\"n\":1}";
        let cleaned = sanitize_control_chars(good);
        assert!(matches!(cleaned, std::borrow::Cow::Borrowed(_)));
        // Escaped sequences (backslash-n) are ordinary bytes and untouched.
        let esc = "{\"a\":\"x\\ny\"}";
        let c2 = sanitize_control_chars(esc);
        let v: serde_json::Value = serde_json::from_str(&c2).unwrap();
        assert_eq!(v["a"], "x\ny");
    }

    #[test]
    fn json_id_accepts_numbers_and_strings() {
        assert_eq!(json_id(&serde_json::json!(216256)), Some(216256));
        assert_eq!(json_id(&serde_json::json!("216256")), Some(216256));
        assert_eq!(json_id(&serde_json::json!(0)), None);
        assert_eq!(json_id(&serde_json::json!("abc")), None);
    }

    #[test]
    fn hex_grid_covers_radius_and_scales() {
        // Small radius -> a single center probe.
        let small = hex_grid(42.0, -76.0, 20.0);
        assert_eq!(small.len(), 1);
        // Large radius -> many probes, all within the radius, bounded.
        let big = hex_grid(42.9317, -76.5661, 320.0);
        assert!(big.len() > 30 && big.len() <= 96);
        for (la, lo, d) in &big {
            let dist = super::haversine_test(42.9317, -76.5661, *la, *lo);
            assert!(dist <= 320.0 + 1.0);
            assert!(*d >= 35.0 && *d <= 90.0);
        }
    }
}

// Small great-circle helper used only by tests above.
#[cfg(test)]
pub(crate) fn haversine_test(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0_f64;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * a.sqrt().atan2((1.0 - a).sqrt())
}
