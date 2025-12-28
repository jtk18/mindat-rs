//! HTTP client for the Mindat API.

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

/// Client for interacting with the Mindat API.
#[derive(Debug, Clone)]
pub struct MindatClient {
    http: Client,
    base_url: Url,
    token: Option<String>,
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

    /// Handle API response.
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            let text = response.text().await?;
            serde_json::from_str(&text).map_err(MindatError::from)
        } else {
            let status_code = status.as_u16();
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

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

    // ==================== Countries ====================

    /// List all countries.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> mindat_rs::Result<()> {
    /// use mindat_rs::MindatClient;
    ///
    /// let client = MindatClient::new("your-token");
    /// let countries = client.countries().await?;
    /// for country in countries.results {
    ///     println!("{}: {}", country.iso, country.text);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn countries(&self) -> Result<PaginatedResponse<Country>> {
        // Note: The /countries/ endpoint may not exist in v1 API
        // Countries are primarily available as filters on the localities endpoint
        self.get("/countries/").await
    }

    /// List countries with pagination.
    pub async fn countries_page(&self, page: i32) -> Result<PaginatedResponse<Country>> {
        #[derive(serde::Serialize)]
        struct Query {
            page: i32,
        }
        // Note: The /countries/ endpoint may not exist in v1 API
        self.get_with_query("/countries/", &Query { page }).await
    }

    /// Get a specific country by ID.
    pub async fn country(&self, id: i32) -> Result<Country> {
        self.get(&format!("/countries/{}/", id)).await
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
        #[derive(serde::Serialize)]
        struct QueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            q: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ima: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            elements_inc: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            elements_exc: Option<String>,
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
            bi_min: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            bi_max: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            optical2v_min: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            optical2v_max: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            varietyof: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            synid: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            polytypeof: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            groupid: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            non_utf: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            meteoritical_code: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            meteoritical_code_exists: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fields: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            omit: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            ordering: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }

        let params = QueryParams {
            name: query.name,
            q: query.q,
            ima: query.ima,
            elements_inc: query.elements_inc,
            elements_exc: query.elements_exc,
            colour: query.colour,
            streak: query.streak,
            hardness_min: query.hardness_min,
            hardness_max: query.hardness_max,
            density_min: query.density_min,
            density_max: query.density_max,
            ri_min: query.ri_min,
            ri_max: query.ri_max,
            bi_min: query.bi_min,
            bi_max: query.bi_max,
            optical2v_min: query.optical2v_min,
            optical2v_max: query.optical2v_max,
            varietyof: query.varietyof,
            synid: query.synid,
            polytypeof: query.polytypeof,
            groupid: query.groupid,
            non_utf: query.non_utf,
            meteoritical_code: query.meteoritical_code,
            meteoritical_code_exists: query.meteoritical_code_exists,
            updated_at: query.updated_at,
            fields: query.fields,
            omit: query.omit,
            ordering: query.ordering.map(|o| o.to_string()),
            page: query.page,
            page_size: query.page_size,
        };

        self.get_with_query("/geomaterials/", &params).await
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
    ) -> Result<CursorPaginatedResponse<Locality>> {
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
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fields: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            omit: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            cursor: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
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
            updated_at: query.updated_at,
            fields: query.fields,
            omit: query.omit,
            cursor: query.cursor,
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

    /// List geographic regions.
    pub async fn geo_regions(
        &self,
        page: Option<i32>,
    ) -> Result<PaginatedResponse<serde_json::Value>> {
        #[derive(serde::Serialize)]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
        }
        self.get_with_query("/locgeoregion2/", &Query { page })
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
            ima: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            updated_at: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fields: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            omit: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page_size: Option<i32>,
        }

        let params = QueryParams {
            q: query.q,
            ima: query.ima,
            updated_at: query.updated_at,
            fields: query.fields,
            omit: query.omit,
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

    // ==================== Other ====================

    /// Get photo count statistics.
    pub async fn photocount(&self) -> Result<serde_json::Value> {
        self.get("/photo-count/").await
    }
}

/// Builder for MindatClient configuration.
#[derive(Debug, Clone)]
pub struct MindatClientBuilder {
    token: Option<String>,
    base_url: String,
    timeout: Option<std::time::Duration>,
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
        let mut client_builder = Client::builder();

        if let Some(timeout) = self.timeout {
            client_builder = client_builder.timeout(timeout);
        }

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
