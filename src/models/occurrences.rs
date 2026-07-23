//! Occurrence types for the Mindat API.
//!
//! Occurrences (`/occurrences/`) are individual mineral-at-locality entries,
//! while occurrence statistics (`/occurrences-statistics/`) aggregate them per
//! mineral/locality pair.

use serde::{Deserialize, Serialize};

use super::serde_helpers::{deserialize_i64, deserialize_optional_i32, deserialize_optional_i64};

/// A single mineral-at-locality occurrence entry (`/occurrences/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Occurrence {
    /// Occurrence (locentry) ID.
    #[serde(deserialize_with = "deserialize_i64")]
    pub id: i64,
    /// Mineral (geomaterial) ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub min: Option<i64>,
    /// Locality ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub loc: Option<i64>,
    /// Type-locality flag/value.
    #[serde(default)]
    pub typeloc: Option<serde_json::Value>,
    /// Questioned flag/value.
    #[serde(default)]
    pub questioned: Option<serde_json::Value>,
    /// Reference ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub refid: Option<i64>,
    /// Description.
    #[serde(default)]
    pub description: Option<String>,
    /// Quality.
    #[serde(default)]
    pub quality: Option<serde_json::Value>,
    /// Rarity.
    #[serde(default)]
    pub rarity: Option<serde_json::Value>,
    /// Habit.
    #[serde(default)]
    pub habit: Option<String>,
    /// Fluorescence.
    #[serde(default)]
    pub fluorescence: Option<String>,
    /// Colour.
    #[serde(default)]
    pub colour: Option<String>,
    /// Confirmation methods.
    #[serde(default)]
    pub cfmethods: Option<String>,
    /// Confirmation reference.
    #[serde(default)]
    pub cfref: Option<String>,
    /// Reference text.
    #[serde(default)]
    pub reftxt: Option<String>,
    /// Date modified.
    #[serde(default)]
    pub datemodify: Option<String>,
    /// Specimen display info.
    #[serde(default)]
    pub specdisp: Option<String>,
    /// Locality reversed-text description.
    #[serde(default)]
    pub lorevtxtd: Option<String>,
}

/// Aggregated occurrence statistics for a mineral/locality pair
/// (`/occurrences-statistics/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccurrenceStatistics {
    /// Locality ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub lstm_loc: Option<i64>,
    /// Mineral (geomaterial) ID.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub lstm_min: Option<i64>,
    /// Number of localities.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub lstm_numlocs: Option<i64>,
    /// IMA-variety flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub lstm_imavar: Option<i32>,
    /// IMA-real flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub lstm_imareal: Option<i32>,
    /// Photo count.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub lstm_photocount: Option<i64>,
    /// Is-locality-entry flag.
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub lstm_islocentry: Option<i64>,
    /// Type-locality value.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub lstm_typeloc: Option<i32>,
    /// Maximum quality.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub lstm_qualitymax: Option<i32>,
    /// Questioned flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub lstm_questioned: Option<i32>,
    /// Sub-localities list.
    #[serde(default)]
    pub lstm_sublocslist: Option<String>,
    /// Display name.
    #[serde(default)]
    pub lstm_display_name: Option<String>,
    /// Last-changed timestamp.
    #[serde(default)]
    pub lstm_changed: Option<String>,
}

/// Builder for `/occurrences/` query parameters.
#[derive(Debug, Clone, Default)]
pub struct OccurrencesQuery {
    /// Filter by mineral (geomaterial) ID.
    pub min: Option<i64>,
    /// Filter by locality ID.
    pub loc: Option<i64>,
    /// Filter by colour.
    pub colour: Option<String>,
    /// Filter by habit.
    pub habit: Option<String>,
    /// Filter by fluorescence.
    pub fluorescence: Option<String>,
    /// Filter by description.
    pub description: Option<String>,
    /// Modified after datetime.
    pub datemodify_after: Option<String>,
    /// Modified before datetime.
    pub datemodify_before: Option<String>,
    /// Ordering field.
    pub ordering: Option<String>,
    /// Fields to include.
    pub fields: Option<String>,
    /// Fields to omit.
    pub omit: Option<String>,
    /// Page number.
    pub page: Option<i32>,
    /// Page size.
    pub page_size: Option<i32>,
}

impl OccurrencesQuery {
    /// Create a new empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter to occurrences of a given mineral.
    pub fn mineral(mut self, mineral_id: i64) -> Self {
        self.min = Some(mineral_id);
        self
    }

    /// Filter to occurrences at a given locality.
    pub fn locality(mut self, locality_id: i64) -> Self {
        self.loc = Some(locality_id);
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

/// Builder for `/occurrences-statistics/` query parameters.
#[derive(Debug, Clone, Default)]
pub struct OccurrenceStatisticsQuery {
    /// Filter by mineral (geomaterial) ID.
    pub lstm_min: Option<i64>,
    /// Filter by locality ID.
    pub lstm_loc: Option<i64>,
    /// Minimum photo count.
    pub photocount_min: Option<i64>,
    /// Maximum photo count.
    pub photocount_max: Option<i64>,
    /// Ordering field.
    pub ordering: Option<String>,
    /// Page number.
    pub page: Option<i32>,
    /// Page size.
    pub page_size: Option<i32>,
}

impl OccurrenceStatisticsQuery {
    /// Create a new empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter to statistics for a given mineral.
    pub fn mineral(mut self, mineral_id: i64) -> Self {
        self.lstm_min = Some(mineral_id);
        self
    }

    /// Filter to statistics for a given locality.
    pub fn locality(mut self, locality_id: i64) -> Self {
        self.lstm_loc = Some(locality_id);
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
