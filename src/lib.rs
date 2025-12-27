//! # mindat-rs
//!
//! A Rust client library for the [Mindat API](https://api.mindat.org/).
//!
//! Mindat is the world's largest open database of minerals, rocks, meteorites, and
//! the localities where they come from. This crate provides a type-safe, async interface
//! to access mineralogical data.
//!
//! ## Features
//!
//! - Full coverage of the Mindat API endpoints
//! - Strongly-typed request builders and response models
//! - Async/await support using tokio
//! - Pagination helpers
//! - Comprehensive error handling
//!
//! ## Quick Start
//!
//! ```no_run
//! use mindat_rs::{MindatClient, GeomaterialsQuery, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Create a client with your API token
//!     let client = MindatClient::new("your-api-token");
//!
//!     // Search for quartz
//!     let query = GeomaterialsQuery::new()
//!         .name("quartz")
//!         .ima_approved(true);
//!
//!     let minerals = client.geomaterials(query).await?;
//!
//!     for mineral in minerals.results {
//!         println!("{}: {:?}", mineral.id, mineral.name);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! Most API endpoints require authentication with a Mindat API token.
//! You can obtain a token from your [Mindat account settings](https://www.mindat.org/).
//!
//! Some endpoints (like `minerals_ima`) can be accessed without authentication:
//!
//! ```no_run
//! use mindat_rs::{MindatClient, ImaMineralsQuery};
//!
//! # async fn example() -> mindat_rs::Result<()> {
//! let client = MindatClient::anonymous();
//! let minerals = client.minerals_ima(ImaMineralsQuery::new()).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Pagination
//!
//! Most list endpoints return paginated results. Use the pagination helpers:
//!
//! ```no_run
//! use mindat_rs::{MindatClient, GeomaterialsQuery};
//!
//! # async fn example() -> mindat_rs::Result<()> {
//! let client = MindatClient::new("token");
//!
//! // Get first page
//! let query = GeomaterialsQuery::new().page(1).page_size(100);
//! let page1 = client.geomaterials(query).await?;
//!
//! // Check if there are more pages
//! if page1.has_next() {
//!     let query = GeomaterialsQuery::new().page(2).page_size(100);
//!     let page2 = client.geomaterials(query).await?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Available Endpoints
//!
//! - **Countries**: List and retrieve country information
//! - **Geomaterials**: Search minerals, rocks, varieties, synonyms, and more
//! - **Localities**: Search mineral localities worldwide
//! - **IMA Minerals**: Access IMA-approved mineral species
//! - **Classification**: Dana 8th ed. and Nickel-Strunz 10th ed. systems
//! - **Locality Metadata**: Ages, statuses, types, and geographic regions

pub mod client;
pub mod error;
pub mod models;

pub use client::{DEFAULT_BASE_URL, MindatClient, MindatClientBuilder};
pub use error::{MindatError, Result};
pub use models::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = MindatClient::new("test-token");
        assert_eq!(client.base_url().as_str(), "https://api.mindat.org/");
    }

    #[test]
    fn test_anonymous_client() {
        let client = MindatClient::anonymous();
        assert_eq!(client.base_url().as_str(), "https://api.mindat.org/");
    }

    #[test]
    fn test_geomaterials_query_builder() {
        let query = GeomaterialsQuery::new()
            .name("quartz")
            .ima_approved(true)
            .with_elements("Si,O")
            .hardness_range(6.0, 7.0)
            .page(1)
            .page_size(50);

        assert_eq!(query.name, Some("quartz".to_string()));
        assert_eq!(query.ima, Some(true));
        assert_eq!(query.elements_inc, Some("Si,O".to_string()));
        assert_eq!(query.hardness_min, Some(6.0));
        assert_eq!(query.hardness_max, Some(7.0));
        assert_eq!(query.page, Some(1));
        assert_eq!(query.page_size, Some(50));
    }

    #[test]
    fn test_localities_query_builder() {
        let query = LocalitiesQuery::new()
            .country("Brazil")
            .with_elements("Au")
            .select_fields("id,txt,country");

        assert_eq!(query.country, Some("Brazil".to_string()));
        assert_eq!(query.elements_inc, Some("Au".to_string()));
        assert_eq!(query.fields, Some("id,txt,country".to_string()));
    }

    #[test]
    fn test_paginated_response() {
        let response: PaginatedResponse<i32> = PaginatedResponse {
            count: Some(150),
            next: Some("https://api.mindat.org/geomaterials/?page=2".to_string()),
            previous: None,
            results: vec![1, 2, 3],
        };

        assert!(response.has_next());
        assert!(!response.has_previous());
        assert_eq!(response.total_pages(50), Some(3));
    }

    #[test]
    fn test_entry_type_from_u8() {
        assert_eq!(EntryType::from(0), EntryType::Mineral);
        assert_eq!(EntryType::from(1), EntryType::Synonym);
        assert_eq!(EntryType::from(2), EntryType::Variety);
        assert_eq!(EntryType::from(7), EntryType::Rock);
        assert_eq!(EntryType::from(99), EntryType::Mineral); // Unknown defaults to Mineral
    }

    #[test]
    fn test_geomaterials_ordering_display() {
        assert_eq!(GeomaterialsOrdering::Id.to_string(), "id");
        assert_eq!(GeomaterialsOrdering::IdDesc.to_string(), "-id");
        assert_eq!(GeomaterialsOrdering::Name.to_string(), "name");
    }
}
