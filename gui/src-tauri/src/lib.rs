use mindat_rs::{GeomaterialsQuery, ImaMineralsQuery, LocalitiesQuery, MindatClient};
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;

/// Debug logging macro - only prints in debug builds
#[cfg(debug_assertions)]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        eprintln!("[Mindat GUI] {}", format!($($arg)*))
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug_log {
    ($($arg:tt)*) => {};
}

/// Application state holding the API client
pub struct AppState {
    client: RwLock<Option<Arc<MindatClient>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: RwLock::new(None),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Error type for commands
#[derive(Debug, Serialize)]
pub struct CommandError {
    message: String,
}

impl From<mindat_rs::MindatError> for CommandError {
    fn from(e: mindat_rs::MindatError) -> Self {
        // Log full error chain for debugging
        debug_log!("Error: {}", e);
        if let mindat_rs::MindatError::Request(ref req_err) = e {
            debug_log!("  Is timeout: {}", req_err.is_timeout());
            debug_log!("  Is connect: {}", req_err.is_connect());
            debug_log!("  Is request: {}", req_err.is_request());
            if let Some(source) = req_err.source() {
                debug_log!("  Source: {}", source);
            }
            if let Some(url) = req_err.url() {
                debug_log!("  URL: {}", url);
            }
        }
        CommandError {
            message: e.to_string(),
        }
    }
}

impl From<String> for CommandError {
    fn from(s: String) -> Self {
        CommandError { message: s }
    }
}

impl From<&str> for CommandError {
    fn from(s: &str) -> Self {
        CommandError {
            message: s.to_string(),
        }
    }
}

type CommandResult<T> = Result<T, CommandError>;

/// Set the API token
#[tauri::command]
async fn set_api_token(state: State<'_, AppState>, token: String) -> CommandResult<String> {
    debug_log!("set_api_token called, token empty: {}", token.is_empty());
    let client = if token.is_empty() {
        MindatClient::anonymous()
    } else {
        MindatClient::new(&token)
    };

    *state.client.write().await = Some(Arc::new(client));
    debug_log!("Client configured successfully");
    Ok("API token set successfully".to_string())
}

/// Check if client is configured
#[tauri::command]
async fn is_configured(state: State<'_, AppState>) -> CommandResult<bool> {
    let configured = state.client.read().await.is_some();
    debug_log!("is_configured called, result: {}", configured);
    Ok(configured)
}

/// Clear the API client
#[tauri::command]
async fn clear_client(state: State<'_, AppState>) -> CommandResult<String> {
    debug_log!("clear_client called");
    *state.client.write().await = None;
    debug_log!("Client cleared");
    Ok("Client cleared".to_string())
}

/// Helper to get client or error
async fn get_client(state: &State<'_, AppState>) -> CommandResult<Arc<MindatClient>> {
    state
        .client
        .read()
        .await
        .clone()
        .ok_or_else(|| "Client not configured. Set an API token first.".into())
}

/// Search for minerals by name
#[tauri::command]
async fn search_minerals(
    state: State<'_, AppState>,
    name: String,
    page: Option<i32>,
    page_size: Option<i32>,
) -> CommandResult<serde_json::Value> {
    debug_log!(
        "search_minerals called: name='{}', page={:?}, page_size={:?}",
        name,
        page,
        page_size
    );
    let client = get_client(&state).await?;

    let mut query = GeomaterialsQuery::new().name(&name);
    if let Some(p) = page {
        query = query.page(p);
    }
    if let Some(ps) = page_size {
        query = query.page_size(ps);
    }

    debug_log!("Executing geomaterials query...");
    let response = client.geomaterials(query).await;
    match &response {
        Ok(r) => debug_log!("Got {} results", r.results.len()),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let response = response?;
    Ok(serde_json::to_value(response).unwrap())
}

/// Search for IMA-approved minerals (no auth required)
#[tauri::command]
async fn search_ima_minerals(
    state: State<'_, AppState>,
    search: String,
    page: Option<i32>,
    page_size: Option<i32>,
) -> CommandResult<serde_json::Value> {
    debug_log!(
        "search_ima_minerals called: search='{}', page={:?}, page_size={:?}",
        search,
        page,
        page_size
    );

    // IMA minerals endpoint works without auth
    let client = match state.client.read().await.clone() {
        Some(c) => {
            debug_log!("Using existing client");
            c
        }
        None => {
            debug_log!("Creating anonymous client");
            Arc::new(MindatClient::anonymous())
        }
    };

    let mut query = ImaMineralsQuery::new();
    if !search.is_empty() {
        query = query.search(&search);
    }
    if let Some(p) = page {
        query = query.page(p);
    }
    if let Some(ps) = page_size {
        query = query.page_size(ps);
    }

    debug_log!("Executing minerals_ima query...");
    let response = client.minerals_ima(query).await;
    match &response {
        Ok(r) => debug_log!("Got {} results", r.results.len()),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let response = response?;
    Ok(serde_json::to_value(response).unwrap())
}

/// Get a specific mineral by ID
#[tauri::command]
async fn get_mineral(state: State<'_, AppState>, id: i32) -> CommandResult<serde_json::Value> {
    debug_log!("get_mineral called: id={}", id);
    let client = get_client(&state).await?;
    debug_log!("Fetching geomaterial...");
    let response = client.geomaterial(id).await;
    match &response {
        Ok(_) => debug_log!("Got mineral"),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let mineral = response?;
    Ok(serde_json::to_value(mineral).unwrap())
}

/// List countries
#[tauri::command]
async fn list_countries(
    state: State<'_, AppState>,
    page: Option<i32>,
) -> CommandResult<serde_json::Value> {
    debug_log!("list_countries called: page={:?}", page);
    let client = get_client(&state).await?;

    debug_log!("Fetching countries...");
    let response = match page {
        Some(p) => client.countries_page(p).await,
        None => client.countries().await,
    };
    match &response {
        Ok(r) => debug_log!("Got {} countries", r.results.len()),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let result = response?;
    Ok(serde_json::to_value(result).unwrap())
}

/// Get a specific country by ID
#[tauri::command]
async fn get_country(state: State<'_, AppState>, id: i32) -> CommandResult<serde_json::Value> {
    debug_log!("get_country called: id={}", id);
    let client = get_client(&state).await?;
    debug_log!("Fetching country...");
    let response = client.country(id).await;
    match &response {
        Ok(_) => debug_log!("Got country"),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let country = response?;
    Ok(serde_json::to_value(country).unwrap())
}

/// Search localities
#[tauri::command]
async fn search_localities(
    state: State<'_, AppState>,
    country: Option<String>,
    name_contains: Option<String>,
) -> CommandResult<serde_json::Value> {
    debug_log!(
        "search_localities called: country={:?}, name_contains={:?}",
        country,
        name_contains
    );
    let client = get_client(&state).await?;

    let mut query = LocalitiesQuery::new();
    if let Some(c) = country {
        query = query.country(&c);
    }
    if let Some(n) = name_contains {
        query = query.name_contains(&n);
    }

    debug_log!("Executing localities query...");
    let response = client.localities(query).await;
    match &response {
        Ok(r) => debug_log!("Got {} localities", r.results.len()),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let response = response?;
    Ok(serde_json::to_value(response).unwrap())
}

/// Get a specific locality by ID
#[tauri::command]
async fn get_locality(state: State<'_, AppState>, id: i32) -> CommandResult<serde_json::Value> {
    debug_log!("get_locality called: id={}", id);
    let client = get_client(&state).await?;
    debug_log!("Fetching locality...");
    let response = client.locality(id).await;
    match &response {
        Ok(_) => debug_log!("Got locality"),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let locality = response?;
    Ok(serde_json::to_value(locality).unwrap())
}

/// Search minerals by elements
#[tauri::command]
async fn search_by_elements(
    state: State<'_, AppState>,
    include_elements: String,
    exclude_elements: Option<String>,
    page: Option<i32>,
) -> CommandResult<serde_json::Value> {
    debug_log!(
        "search_by_elements called: include='{}', exclude={:?}, page={:?}",
        include_elements,
        exclude_elements,
        page
    );
    let client = get_client(&state).await?;

    let mut query = GeomaterialsQuery::new().with_elements(&include_elements);
    if let Some(exclude) = exclude_elements {
        query = query.without_elements(&exclude);
    }
    if let Some(p) = page {
        query = query.page(p);
    }

    debug_log!("Executing geomaterials query by elements...");
    let response = client.geomaterials(query).await;
    match &response {
        Ok(r) => debug_log!("Got {} results", r.results.len()),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let response = response?;
    Ok(serde_json::to_value(response).unwrap())
}

/// Search localities by GPS coordinates (bounding box search)
/// Requires at least a country filter to avoid fetching all localities
#[tauri::command]
async fn search_localities_by_gps(
    state: State<'_, AppState>,
    latitude: f64,
    longitude: f64,
    radius_km: f64,
    country: Option<String>,
    name_contains: Option<String>,
) -> CommandResult<serde_json::Value> {
    debug_log!(
        "search_localities_by_gps called: lat={}, lon={}, radius={}km, country={:?}, name={:?}",
        latitude,
        longitude,
        radius_km,
        country,
        name_contains
    );

    // Require at least one filter to avoid fetching all localities (which times out)
    if country.is_none() && name_contains.is_none() {
        return Err("Please specify a country or name filter to narrow down the search".into());
    }

    let client = get_client(&state).await?;

    // Calculate bounding box from center point and radius
    // 1 degree of latitude ~ 111 km
    // 1 degree of longitude ~ 111 km * cos(latitude)
    let lat_delta = radius_km / 111.0;
    let lon_delta = radius_km / (111.0 * latitude.to_radians().cos().abs().max(0.01));

    let min_lat = latitude - lat_delta;
    let max_lat = latitude + lat_delta;
    let min_lon = longitude - lon_delta;
    let max_lon = longitude + lon_delta;

    debug_log!(
        "Bounding box: lat=[{}, {}], lon=[{}, {}]",
        min_lat,
        max_lat,
        min_lon,
        max_lon
    );

    // Build query with filters
    let mut query = LocalitiesQuery::new();
    if let Some(c) = country {
        if !c.is_empty() {
            query = query.country(&c);
        }
    }
    if let Some(n) = name_contains {
        if !n.is_empty() {
            query = query.name_contains(&n);
        }
    }

    debug_log!("Fetching localities with filters...");
    let response = client.localities(query).await;
    match &response {
        Ok(r) => debug_log!("Got {} localities, filtering by GPS...", r.results.len()),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let mut response = response?;

    // Filter results to only include localities with GPS within bounding box
    response.results.retain(|loc| {
        if let (Some(lat), Some(lon)) = (loc.latitude, loc.longitude) {
            lat >= min_lat && lat <= max_lat && lon >= min_lon && lon <= max_lon
        } else {
            false
        }
    });

    debug_log!(
        "Found {} localities within {}km",
        response.results.len(),
        radius_km
    );
    Ok(serde_json::to_value(response).unwrap())
}

/// Search localities by elements
#[tauri::command]
async fn search_localities_by_elements(
    state: State<'_, AppState>,
    include_elements: String,
    exclude_elements: Option<String>,
) -> CommandResult<serde_json::Value> {
    debug_log!(
        "search_localities_by_elements called: include='{}', exclude={:?}",
        include_elements,
        exclude_elements
    );
    let client = get_client(&state).await?;

    let mut query = LocalitiesQuery::new().with_elements(&include_elements);
    if let Some(exclude) = exclude_elements {
        query = query.without_elements(&exclude);
    }

    debug_log!("Executing localities query by elements...");
    let response = client.localities(query).await;
    match &response {
        Ok(r) => debug_log!("Got {} localities", r.results.len()),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let mut response = response?;

    // Filter to only show localities with GPS coordinates
    response
        .results
        .retain(|loc| loc.latitude.is_some() && loc.longitude.is_some());
    debug_log!("{} localities have GPS coordinates", response.results.len());

    Ok(serde_json::to_value(response).unwrap())
}

/// Get Dana-8 classification groups
#[tauri::command]
async fn get_dana8_groups(state: State<'_, AppState>) -> CommandResult<serde_json::Value> {
    debug_log!("get_dana8_groups called");
    let client = get_client(&state).await?;
    debug_log!("Fetching Dana-8 groups...");
    let response = client.dana8_groups().await;
    match &response {
        Ok(_) => debug_log!("Got Dana-8 groups"),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let groups = response?;
    Ok(groups)
}

/// Get Strunz-10 classification classes
#[tauri::command]
async fn get_strunz10_classes(state: State<'_, AppState>) -> CommandResult<serde_json::Value> {
    debug_log!("get_strunz10_classes called");
    let client = get_client(&state).await?;
    debug_log!("Fetching Strunz-10 classes...");
    let response = client.strunz10_classes().await;
    match &response {
        Ok(_) => debug_log!("Got Strunz-10 classes"),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let classes = response?;
    Ok(classes)
}

/// Quick search for geomaterials
#[tauri::command]
async fn quick_search(
    state: State<'_, AppState>,
    query: String,
    size: Option<i32>,
) -> CommandResult<serde_json::Value> {
    debug_log!("quick_search called: query='{}', size={:?}", query, size);
    let client = get_client(&state).await?;
    debug_log!("Executing quick search...");
    let response = client.geomaterials_search(&query, size).await;
    match &response {
        Ok(r) => debug_log!("Got {} results", r.len()),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let results = response?;
    Ok(serde_json::to_value(results).unwrap())
}

/// Get photo count statistics
#[tauri::command]
async fn get_photo_count(state: State<'_, AppState>) -> CommandResult<serde_json::Value> {
    debug_log!("get_photo_count called");
    let client = get_client(&state).await?;
    debug_log!("Fetching photo count...");
    let response = client.photocount().await;
    match &response {
        Ok(_) => debug_log!("Got photo count"),
        Err(e) => debug_log!("Query failed: {}", e),
    }
    let count = response?;
    Ok(count)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    debug_log!("Starting Mindat Explorer GUI...");

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            set_api_token,
            is_configured,
            clear_client,
            search_minerals,
            search_ima_minerals,
            get_mineral,
            list_countries,
            get_country,
            search_localities,
            search_localities_by_gps,
            search_localities_by_elements,
            get_locality,
            search_by_elements,
            get_dana8_groups,
            get_strunz10_classes,
            quick_search,
            get_photo_count,
        ]);

    #[cfg(desktop)]
    let builder = builder.plugin(tauri_plugin_opener::init());

    debug_log!("Tauri builder configured, starting app...");

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
