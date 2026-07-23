//! Mineral relation types for the Mindat API (`/relations/`).

use serde::{Deserialize, Serialize};

use super::serde_helpers::{deserialize_i64, deserialize_optional_i64};

/// A relation between two minerals (`/relations/`).
///
/// Not to be confused with [`crate::Relation`], which is the embedded relation
/// object returned inside a geomaterial record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MineralRelation {
    /// Relation ID.
    #[serde(deserialize_with = "deserialize_i64")]
    pub rid: i64,
    /// First mineral (geomaterial) ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub min1: Option<i64>,
    /// Second mineral (geomaterial) ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub min2: Option<i64>,
    /// Relation type.
    #[serde(default)]
    pub rel: Option<serde_json::Value>,
}

/// Builder for `/relations/` query parameters.
#[derive(Debug, Clone, Default)]
pub struct RelationsQuery {
    /// Free-text search.
    pub q: Option<String>,
    /// Filter by first mineral ID.
    pub min1: Option<i64>,
    /// Filter by second mineral ID.
    pub min2: Option<i64>,
    /// Filter by relation type.
    pub rel: Option<i32>,
    /// Page number.
    pub page: Option<i32>,
    /// Page size.
    pub page_size: Option<i32>,
}

impl RelationsQuery {
    /// Create a new empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter to relations involving a given mineral (as `min1`).
    pub fn mineral(mut self, mineral_id: i64) -> Self {
        self.min1 = Some(mineral_id);
        self
    }

    /// Set page number.
    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    /// Set page size.
    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }
}
