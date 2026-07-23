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
///
/// As of the current Mindat API, `/minerals-ima/` is a full search endpoint
/// (not just a plain list), supporting the same style of filters as
/// `/geomaterials/` — and it works without an API key.
#[derive(Debug, Clone, Default)]
pub struct ImaMineralsQuery {
    /// Search query.
    pub q: Option<String>,
    /// Name filter (supports `*` and `_` wildcards).
    pub name: Option<String>,
    /// IMA filter.
    pub ima: Option<i32>,
    /// Include elements (comma-separated); sent as `el_inc`.
    pub elements_inc: Option<String>,
    /// Exclude elements (comma-separated); sent as `el_exc`.
    pub elements_exc: Option<String>,
    /// Essential elements (comma-separated); sent as `el_essential`.
    pub el_essential: Option<String>,
    /// Colour filter.
    pub colour: Option<String>,
    /// Streak filter.
    pub streak: Option<String>,
    /// Hardness minimum (Mohs).
    pub hardness_min: Option<f32>,
    /// Hardness maximum (Mohs).
    pub hardness_max: Option<f32>,
    /// Density minimum.
    pub density_min: Option<f64>,
    /// Density maximum.
    pub density_max: Option<f64>,
    /// Refractive index minimum.
    pub ri_min: Option<f32>,
    /// Refractive index maximum.
    pub ri_max: Option<f32>,
    /// Variety of (geomaterial ID).
    pub varietyof: Option<i32>,
    /// Synonym of (geomaterial ID).
    pub synid: Option<i32>,
    /// Polytype of (geomaterial ID).
    pub polytypeof: Option<i32>,
    /// Group ID.
    pub groupid: Option<i32>,
    /// Meteoritical code filter.
    pub meteoritical_code: Option<String>,
    /// Whether a meteoritical code exists.
    pub meteoritical_code_exists: Option<bool>,
    /// Include non-UTF names.
    pub non_utf: Option<bool>,
    /// Filter by IDs.
    pub id_in: Option<Vec<i32>>,
    /// Minimum ID.
    pub id_min: Option<i32>,
    /// Maximum ID.
    pub id_max: Option<i32>,
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

    /// Filter by name (supports `*` and `_` wildcards).
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Filter by included elements.
    pub fn with_elements(mut self, elements: impl Into<String>) -> Self {
        self.elements_inc = Some(elements.into());
        self
    }

    /// Filter by excluded elements.
    pub fn without_elements(mut self, elements: impl Into<String>) -> Self {
        self.elements_exc = Some(elements.into());
        self
    }

    /// Filter by essential elements.
    pub fn essential_elements(mut self, elements: impl Into<String>) -> Self {
        self.el_essential = Some(elements.into());
        self
    }

    /// Filter by hardness range (Mohs).
    pub fn hardness_range(mut self, min: f32, max: f32) -> Self {
        self.hardness_min = Some(min);
        self.hardness_max = Some(max);
        self
    }

    /// Filter by density range.
    pub fn density_range(mut self, min: f64, max: f64) -> Self {
        self.density_min = Some(min);
        self.density_max = Some(max);
        self
    }

    /// Filter by a set of geomaterial IDs.
    pub fn ids(mut self, ids: Vec<i32>) -> Self {
        self.id_in = Some(ids);
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
