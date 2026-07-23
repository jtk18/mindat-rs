//! MCP Server for the Mindat mineralogical database
//!
//! This server exposes the mindat-rs library functionality through the Model Context Protocol,
//! allowing Claude Desktop and other MCP clients to search for minerals, localities, and more.

use mindat_rs::{
    models::{GeomaterialsQuery, ImaMineralsQuery, LocalitiesQuery, ReferencesQuery},
    MindatClient,
};
use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars::JsonSchema,
    tool, tool_handler, tool_router,
    transport::stdio,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Request parameters for searching minerals by name
#[derive(Debug, Deserialize, JsonSchema)]
struct SearchMineralsRequest {
    /// The mineral name or partial name to search for
    name: String,
    /// Page number (starts at 1)
    #[serde(default)]
    page: Option<i32>,
    /// Number of results per page (max 50)
    #[serde(default)]
    page_size: Option<i32>,
}

/// Request parameters for searching IMA-approved minerals
#[derive(Debug, Deserialize, JsonSchema)]
struct SearchImaMineralsRequest {
    /// Search query for mineral name
    query: String,
    /// Page number (starts at 1)
    #[serde(default)]
    page: Option<i32>,
    /// Number of results per page (max 50)
    #[serde(default)]
    page_size: Option<i32>,
}

/// Request parameters for getting a specific mineral by ID
#[derive(Debug, Deserialize, JsonSchema)]
struct GetMineralRequest {
    /// The Mindat mineral ID
    id: i32,
}

/// Request parameters for searching minerals by chemical elements
#[derive(Debug, Deserialize, JsonSchema)]
struct SearchByElementsRequest {
    /// Elements that must be present (e.g., ["Cu", "Fe", "S"])
    include_elements: Vec<String>,
    /// Elements that must NOT be present (optional)
    #[serde(default)]
    exclude_elements: Option<Vec<String>>,
    /// Page number (starts at 1)
    #[serde(default)]
    page: Option<i32>,
}

/// Request parameters for searching localities
#[derive(Debug, Deserialize, JsonSchema)]
struct SearchLocalitiesRequest {
    /// Country name (use abbreviations like "USA", "UK")
    #[serde(default)]
    country: Option<String>,
    /// Filter localities by name containing this string
    #[serde(default)]
    name_contains: Option<String>,
    /// Elements that must be found at the locality
    #[serde(default)]
    include_elements: Option<Vec<String>>,
    /// Elements that must NOT be found at the locality
    #[serde(default)]
    exclude_elements: Option<Vec<String>>,
}

/// Request parameters for GPS-based locality search
#[derive(Debug, Deserialize, JsonSchema)]
struct SearchLocalitiesByGpsRequest {
    /// Latitude in decimal degrees
    latitude: f64,
    /// Longitude in decimal degrees
    longitude: f64,
    /// Search radius in kilometers
    radius_km: f64,
    /// Optional country filter to narrow results
    #[serde(default)]
    country: Option<String>,
    /// Optional name filter
    #[serde(default)]
    name_contains: Option<String>,
}

/// Request parameters for getting a specific locality by ID
#[derive(Debug, Deserialize, JsonSchema)]
struct GetLocalityRequest {
    /// The Mindat locality ID
    id: i32,
}

/// Request parameters for searching references
#[derive(Debug, Deserialize, JsonSchema)]
struct SearchReferencesRequest {
    /// Free-text search across references (title, journal, author, etc.)
    #[serde(default)]
    query: Option<String>,
    /// Page number (starts at 1)
    #[serde(default)]
    page: Option<i32>,
    /// Number of results per page
    #[serde(default)]
    page_size: Option<i32>,
}

/// Request parameters for quick search
#[derive(Debug, Deserialize, JsonSchema)]
struct QuickSearchRequest {
    /// Search query
    query: String,
    /// Maximum number of results (default 10)
    #[serde(default)]
    size: Option<i32>,
}

/// The Mindat MCP service
#[derive(Clone)]
pub struct MindatService {
    client: Arc<Mutex<MindatClient>>,
    tool_router: ToolRouter<Self>,
}

impl MindatService {
    pub fn new() -> Self {
        // Try to get API key from environment, fall back to anonymous
        let client = match std::env::var("MINDAT_API_KEY") {
            Ok(key) if !key.is_empty() => MindatClient::new(key),
            _ => MindatClient::anonymous(),
        };

        Self {
            client: Arc::new(Mutex::new(client)),
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl MindatService {
    /// Search for minerals by name in the Mindat database.
    #[tool(description = "Search for minerals by name in the Mindat database. Returns minerals matching the search query with properties like chemical formula, crystal system, hardness, and locality information.")]
    async fn search_minerals(
        &self,
        Parameters(req): Parameters<SearchMineralsRequest>,
    ) -> String {
        let client = self.client.lock().await;
        let page = req.page.unwrap_or(1);
        let page_size = req.page_size.unwrap_or(20);
        let query = GeomaterialsQuery::new()
            .name(&req.name)
            .page(page)
            .page_size(page_size);

        match client.geomaterials(query).await {
            Ok(response) => {
                let result = serde_json::json!({
                    "count": response.count,
                    "page": page,
                    "total_pages": response.total_pages(page_size as usize),
                    "results": response.results
                });
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| "Error formatting response".to_string())
            }
            Err(e) => format!("Error searching minerals: {}", e),
        }
    }

    /// Search for IMA-approved minerals. This endpoint works without authentication.
    #[tool(description = "Search for IMA (International Mineralogical Association) approved minerals. This works without an API key and returns official mineral names, formulas, symbols, and approval information.")]
    async fn search_ima_minerals(
        &self,
        Parameters(req): Parameters<SearchImaMineralsRequest>,
    ) -> String {
        let client = self.client.lock().await;
        let page = req.page.unwrap_or(1);
        let page_size = req.page_size.unwrap_or(20);
        let query = ImaMineralsQuery::new()
            .search(&req.query)
            .page(page)
            .page_size(page_size);

        match client.minerals_ima(query).await {
            Ok(response) => {
                let result = serde_json::json!({
                    "count": response.count,
                    "page": page,
                    "total_pages": response.total_pages(page_size as usize),
                    "results": response.results
                });
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| "Error formatting response".to_string())
            }
            Err(e) => format!("Error searching IMA minerals: {}", e),
        }
    }

    /// Get detailed information about a specific mineral by its Mindat ID.
    #[tool(description = "Get detailed information about a specific mineral by its Mindat ID. Returns comprehensive data including chemical formula, physical properties, crystal system, optical properties, and related minerals.")]
    async fn get_mineral(
        &self,
        Parameters(req): Parameters<GetMineralRequest>,
    ) -> String {
        let client = self.client.lock().await;

        match client.geomaterial(req.id).await {
            Ok(mineral) => serde_json::to_string_pretty(&mineral).unwrap_or_else(|_| "Error formatting response".to_string()),
            Err(e) => format!("Error getting mineral {}: {}", req.id, e),
        }
    }

    /// Search for minerals containing specific chemical elements.
    #[tool(description = "Search for minerals by their chemical composition. Specify elements that must be present and optionally elements that must be absent. Uses standard element symbols like 'Cu', 'Fe', 'S'.")]
    async fn search_by_elements(
        &self,
        Parameters(req): Parameters<SearchByElementsRequest>,
    ) -> String {
        let client = self.client.lock().await;
        let page = req.page.unwrap_or(1);

        let include_str = req.include_elements.join(",");
        let mut query = GeomaterialsQuery::new()
            .ima_approved(true)
            .with_elements(&include_str)
            .page(page)
            .page_size(20);

        if let Some(ref exclude) = req.exclude_elements {
            if !exclude.is_empty() {
                let exclude_str = exclude.join(",");
                query = query.without_elements(&exclude_str);
            }
        }

        match client.geomaterials(query).await {
            Ok(response) => {
                let result = serde_json::json!({
                    "count": response.count,
                    "page": page,
                    "total_pages": response.total_pages(20),
                    "results": response.results
                });
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| "Error formatting response".to_string())
            }
            Err(e) => format!("Error searching by elements: {}", e),
        }
    }

    /// Quick search for minerals - returns fast autocomplete-style results.
    #[tool(description = "Perform a quick autocomplete-style search for minerals. Returns fast results suitable for searching as you type.")]
    async fn quick_search(
        &self,
        Parameters(req): Parameters<QuickSearchRequest>,
    ) -> String {
        let client = self.client.lock().await;
        let size = req.size.unwrap_or(10);

        match client.geomaterials_search(&req.query, Some(size)).await {
            Ok(results) => serde_json::to_string_pretty(&results).unwrap_or_else(|_| "Error formatting response".to_string()),
            Err(e) => format!("Error in quick search: {}", e),
        }
    }

    /// Search for mineral localities (mining sites, outcrops, etc.)
    #[tool(description = "Search for mineral localities (mines, outcrops, quarries, etc.) by country, name, or elements found there. Use country abbreviations like 'USA', 'UK'. Returns location coordinates and mineral information.")]
    async fn search_localities(
        &self,
        Parameters(req): Parameters<SearchLocalitiesRequest>,
    ) -> String {
        let client = self.client.lock().await;

        let mut query = LocalitiesQuery::new();

        if let Some(ref country) = req.country {
            query = query.country(country);
        }
        if let Some(ref name) = req.name_contains {
            query = query.name_contains(name);
        }
        if let Some(ref include) = req.include_elements {
            if !include.is_empty() {
                let include_str = include.join(",");
                query = query.with_elements(&include_str);
            }
        }
        if let Some(ref exclude) = req.exclude_elements {
            if !exclude.is_empty() {
                let exclude_str = exclude.join(",");
                query = query.without_elements(&exclude_str);
            }
        }

        match client.localities(query).await {
            Ok(response) => {
                let result = serde_json::json!({
                    "results": response.results
                });
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| "Error formatting response".to_string())
            }
            Err(e) => format!("Error searching localities: {}", e),
        }
    }

    /// Search for localities near a GPS coordinate.
    #[tool(description = "Search for mineral localities near a specific GPS coordinate. Specify latitude, longitude, and search radius in kilometers. Add country or name filters to narrow results and avoid timeouts.")]
    async fn search_localities_by_gps(
        &self,
        Parameters(req): Parameters<SearchLocalitiesByGpsRequest>,
    ) -> String {
        let client = self.client.lock().await;

        // Calculate bounding box
        let km_per_degree_lat = 111.0;
        let km_per_degree_lon = 111.0 * (req.latitude.to_radians().cos());
        let delta_lat = req.radius_km / km_per_degree_lat;
        let delta_lon = if km_per_degree_lon > 0.0 {
            req.radius_km / km_per_degree_lon
        } else {
            req.radius_km / km_per_degree_lat
        };

        let min_lat = req.latitude - delta_lat;
        let max_lat = req.latitude + delta_lat;
        let min_lon = req.longitude - delta_lon;
        let max_lon = req.longitude + delta_lon;

        let mut all_results = Vec::new();
        let max_pages = 10; // Limit to avoid timeout

        for page in 1..=max_pages {
            let mut query = LocalitiesQuery::new().page(page);

            if let Some(ref country) = req.country {
                query = query.country(country);
            }
            if let Some(ref name) = req.name_contains {
                query = query.name_contains(name);
            }

            match client.localities(query).await {
                Ok(response) => {
                    // Filter results within the radius
                    for loc in &response.results {
                        if let (Some(lat), Some(lon)) = (loc.latitude, loc.longitude) {
                            if lat >= min_lat && lat <= max_lat && lon >= min_lon && lon <= max_lon {
                                // Check actual distance using Haversine formula
                                let dlat = (lat - req.latitude).to_radians();
                                let dlon = (lon - req.longitude).to_radians();
                                let a = (dlat / 2.0).sin().powi(2)
                                    + req.latitude.to_radians().cos()
                                        * lat.to_radians().cos()
                                        * (dlon / 2.0).sin().powi(2);
                                let c = 2.0 * a.sqrt().asin();
                                let distance = 6371.0 * c;

                                if distance <= req.radius_km {
                                    all_results.push(serde_json::json!({
                                        "locality": loc,
                                        "distance_km": (distance * 100.0).round() / 100.0
                                    }));
                                }
                            }
                        }
                    }

                    if !response.has_next() {
                        break;
                    }
                }
                Err(e) => {
                    return format!("Error searching localities: {}", e);
                }
            }
        }

        let result = serde_json::json!({
            "center": {
                "latitude": req.latitude,
                "longitude": req.longitude
            },
            "radius_km": req.radius_km,
            "count": all_results.len(),
            "results": all_results
        });
        serde_json::to_string_pretty(&result).unwrap_or_else(|_| "Error formatting response".to_string())
    }

    /// Get detailed information about a specific locality.
    #[tool(description = "Get detailed information about a specific locality by its Mindat ID. Returns location data, coordinates, minerals found there, and administrative information.")]
    async fn get_locality(
        &self,
        Parameters(req): Parameters<GetLocalityRequest>,
    ) -> String {
        let client = self.client.lock().await;

        match client.locality(req.id).await {
            Ok(locality) => serde_json::to_string_pretty(&locality).unwrap_or_else(|_| "Error formatting response".to_string()),
            Err(e) => format!("Error getting locality {}: {}", req.id, e),
        }
    }

    /// Search bibliographic references.
    #[tool(description = "Search the Mindat literature/reference database by free text (title, journal, author, etc.). Returns bibliographic records. Requires an API key.")]
    async fn search_references(
        &self,
        Parameters(req): Parameters<SearchReferencesRequest>,
    ) -> String {
        let client = self.client.lock().await;
        let mut query = ReferencesQuery::new();
        if let Some(ref q) = req.query {
            query = query.search(q);
        }
        if let Some(page) = req.page {
            query = query.page(page);
        }
        if let Some(page_size) = req.page_size {
            query = query.page_size(page_size);
        }

        match client.references(query).await {
            Ok(response) => {
                let result = serde_json::json!({
                    "count": response.count,
                    "results": response.results
                });
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| "Error formatting response".to_string())
            }
            Err(e) => format!("Error searching references: {}", e),
        }
    }

    /// Get Dana 8th edition classification groups.
    #[tool(description = "Get the Dana 8th edition mineral classification system groups. This is a hierarchical classification system based on mineral chemistry and structure.")]
    async fn get_dana_groups(&self) -> String {
        let client = self.client.lock().await;

        match client.dana8_groups().await {
            Ok(groups) => serde_json::to_string_pretty(&groups).unwrap_or_else(|_| "Error formatting response".to_string()),
            Err(e) => format!("Error getting Dana groups: {}", e),
        }
    }

    /// Get Nickel-Strunz 10th edition classification classes.
    #[tool(description = "Get the Nickel-Strunz 10th edition mineral classification system classes. This is an internationally recognized classification system for minerals.")]
    async fn get_strunz_classes(&self) -> String {
        let client = self.client.lock().await;

        match client.strunz10_classes().await {
            Ok(classes) => serde_json::to_string_pretty(&classes).unwrap_or_else(|_| "Error formatting response".to_string()),
            Err(e) => format!("Error getting Strunz classes: {}", e),
        }
    }

}

#[tool_handler]
impl ServerHandler for MindatService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Mindat MCP Server - Access the world's largest open database of minerals, rocks, and localities. \
                Use 'search_ima_minerals' for searches without an API key. \
                Set MINDAT_API_KEY environment variable for full access to all endpoints."
                    .to_string(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for debugging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Starting Mindat MCP server");

    let service = MindatService::new();
    let server = service.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("Failed to start server: {}", e);
    })?;

    tracing::info!("Mindat MCP server running");
    server.waiting().await?;

    Ok(())
}
