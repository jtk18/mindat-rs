//! IMA Mineral types for the Mindat API.

use serde::{Deserialize, Serialize};

use super::serde_helpers::{deserialize_optional_vec_i32, deserialize_optional_vec_string};

/// An IMA-approved mineral from the Mindat database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImaMaterial {
    /// Mindat ID.
    pub id: i32,
    /// Mineral name.
    #[serde(default)]
    pub name: Option<String>,
    /// IMA-approved formula.
    #[serde(default)]
    pub ima_formula: Option<String>,
    /// IMA symbol (3 letters).
    #[serde(default)]
    pub ima_symbol: Option<String>,
    /// IMA approval year.
    #[serde(default)]
    pub ima_year: Option<String>,
    /// Discovery year.
    #[serde(default)]
    pub discovery_year: Option<String>,
    /// IMA status values.
    #[serde(default, deserialize_with = "deserialize_optional_vec_string")]
    pub ima_status: Option<Vec<String>>,
    /// IMA notes.
    #[serde(default, deserialize_with = "deserialize_optional_vec_string")]
    pub ima_notes: Option<Vec<String>>,
    /// Type specimen storage location.
    #[serde(default)]
    pub type_specimen_store: Option<String>,
    /// Mindat long ID.
    #[serde(default)]
    pub mindat_longid: Option<String>,
    /// Mindat GUID.
    #[serde(default)]
    pub mindat_guid: Option<String>,
    /// Type localities.
    #[serde(default, deserialize_with = "deserialize_optional_vec_i32")]
    pub type_localities: Option<Vec<i32>>,
    /// Short description.
    #[serde(default)]
    pub description_short: Option<String>,
    /// Mindat formula.
    #[serde(default)]
    pub mindat_formula: Option<String>,
    /// Mindat formula note.
    #[serde(default)]
    pub mindat_formula_note: Option<String>,
}

/// Builder for IMA minerals query parameters.
#[derive(Debug, Clone, Default)]
pub struct ImaMineralsQuery {
    /// Search query.
    pub q: Option<String>,
    /// IMA filter.
    pub ima: Option<i32>,
    /// Filter by IDs.
    pub id_in: Option<Vec<i32>>,
    /// Updated after datetime.
    pub updated_at: Option<String>,
    /// Fields to include.
    pub fields: Option<String>,
    /// Fields to omit.
    pub omit: Option<String>,
    /// Fields to expand.
    pub expand: Option<Vec<String>>,
    /// Page number.
    pub page: Option<i32>,
    /// Page size.
    pub page_size: Option<i32>,
}

impl ImaMineralsQuery {
    /// Create a new empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Search query.
    pub fn search(mut self, q: impl Into<String>) -> Self {
        self.q = Some(q.into());
        self
    }

    /// Select specific fields.
    pub fn select_fields(mut self, fields: impl Into<String>) -> Self {
        self.fields = Some(fields.into());
        self
    }

    /// Omit specific fields.
    pub fn omit_fields(mut self, fields: impl Into<String>) -> Self {
        self.omit = Some(fields.into());
        self
    }

    /// Expand related fields.
    pub fn expand_fields(mut self, fields: Vec<String>) -> Self {
        self.expand = Some(fields);
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
