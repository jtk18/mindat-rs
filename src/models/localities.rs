//! Locality types for the Mindat API.

use serde::{Deserialize, Serialize};

use super::serde_helpers::{
    deserialize_optional_f64, deserialize_optional_i16, deserialize_optional_i32,
    deserialize_optional_vec_i32,
};

/// A locality from the Mindat database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locality {
    /// Mindat ID.
    pub id: i32,
    /// Long ID string.
    #[serde(default)]
    pub longid: Option<String>,
    /// GUID.
    #[serde(default)]
    pub guid: Option<String>,
    /// Locality text/name.
    #[serde(default)]
    pub txt: Option<String>,
    /// Reversed text description.
    #[serde(default)]
    pub revtxtd: Option<String>,
    /// Short description.
    #[serde(default)]
    pub description_short: Option<String>,
    /// Latitude.
    #[serde(default, deserialize_with = "deserialize_optional_f64")]
    pub latitude: Option<f64>,
    /// Longitude.
    #[serde(default, deserialize_with = "deserialize_optional_f64")]
    pub longitude: Option<f64>,
    /// Language-specific text.
    #[serde(default)]
    pub langtxt: Option<String>,
    /// Date added.
    #[serde(default)]
    pub dateadd: Option<String>,
    /// Date modified.
    #[serde(default)]
    pub datemodify: Option<String>,
    /// Elements found at this locality.
    #[serde(default)]
    pub elements: Option<String>,
    /// Country name.
    #[serde(default)]
    pub country: Option<String>,
    /// References.
    #[serde(default)]
    pub refs: Option<String>,
    /// Coordinate system.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub coordsystem: Option<i32>,
    /// Parent locality ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub parent: Option<i32>,
    /// Links.
    #[serde(default)]
    pub links: Option<String>,
    /// Area.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub area: Option<i32>,
    /// Non-hierarchical flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub non_hierarchical: Option<i32>,
    /// Age ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub age: Option<i32>,
    /// Meteorite type.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub meteorite_type: Option<i32>,
    /// Company ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub company: Option<i32>,
    /// Company 2 ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub company2: Option<i32>,
    /// Locality status ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub loc_status: Option<i32>,
    /// Locality group.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub loc_group: Option<i32>,
    /// Status year.
    #[serde(default)]
    pub status_year: Option<String>,
    /// Company year.
    #[serde(default)]
    pub company_year: Option<String>,
    /// Discovered before.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub discovered_before: Option<i32>,
    /// Discovery year.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub discovery_year: Option<i32>,
    /// Discovery year type.
    #[serde(default)]
    pub discovery_year_type: Option<String>,
    /// Hierarchy level.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub level: Option<i32>,
    /// Included localities.
    #[serde(default)]
    pub locsinclude: Option<String>,
    /// Excluded localities.
    #[serde(default)]
    pub locsexclude: Option<String>,
    /// Wikipedia link.
    #[serde(default)]
    pub wikipedia: Option<String>,
    /// OSM ID.
    #[serde(default)]
    pub osmid: Option<String>,
    /// Geonames ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub geonames: Option<i32>,
    /// Timestamp.
    #[serde(default)]
    pub timestamp: Option<String>,
    /// Geomaterials at this locality (when expanded).
    #[serde(default, deserialize_with = "deserialize_optional_vec_i32")]
    pub geomaterials: Option<Vec<i32>>,
}

/// Builder for locality query parameters.
#[derive(Debug, Clone, Default)]
pub struct LocalitiesQuery {
    /// Country name filter.
    pub country: Option<String>,
    /// Locality name contains.
    pub txt: Option<String>,
    /// Description contains.
    pub description: Option<String>,
    /// Include elements (comma-separated).
    pub elements_inc: Option<String>,
    /// Exclude elements (comma-separated).
    pub elements_exc: Option<String>,
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
    /// Cursor for pagination.
    pub cursor: Option<String>,
}

impl LocalitiesQuery {
    /// Create a new empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by country name.
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Filter by locality name.
    pub fn name_contains(mut self, txt: impl Into<String>) -> Self {
        self.txt = Some(txt.into());
        self
    }

    /// Filter by description.
    pub fn description_contains(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
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

    /// Set cursor for pagination.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

/// Locality age information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalityAge {
    /// Age ID.
    pub age_id: i32,
    /// Age MA value.
    #[serde(default, deserialize_with = "deserialize_optional_f64")]
    pub age_mav: Option<f64>,
    /// Age PM value.
    #[serde(default, deserialize_with = "deserialize_optional_f64")]
    pub age_pmv: Option<f64>,
    /// Age MA2 value.
    #[serde(default, deserialize_with = "deserialize_optional_f64")]
    pub age_ma2v: Option<f64>,
    /// Age PM2 value.
    #[serde(default, deserialize_with = "deserialize_optional_f64")]
    pub age_pm2v: Option<f64>,
    /// Age method.
    #[serde(default)]
    pub agemethod: Option<String>,
    /// Age reference.
    #[serde(default)]
    pub agereference: Option<String>,
    /// Age MA string.
    #[serde(default)]
    pub age_ma: Option<String>,
    /// Age PM string.
    #[serde(default)]
    pub age_pm: Option<String>,
    /// Age MA2 string.
    #[serde(default)]
    pub age_ma2: Option<String>,
    /// Age PM2 string.
    #[serde(default)]
    pub age_pm2: Option<String>,
    /// Ages 1.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ages1: Option<i32>,
    /// Ages 2.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ages2: Option<i32>,
    /// Age type.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub age_type: Option<i32>,
}

/// Locality status information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalityStatus {
    /// Status ID.
    pub ls_id: i32,
    /// Status text.
    pub ls_text: String,
    /// Historical flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ls_historical: Option<i32>,
    /// Wide flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub ls_wide: Option<i32>,
}

/// Locality type information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalityType {
    /// Type ID.
    pub lt_id: i32,
    /// Type text.
    pub lt_text: String,
    /// Parent type ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub lt_parent: Option<i32>,
    /// Sort order.
    #[serde(default, deserialize_with = "deserialize_optional_i16")]
    pub lt_sortorder: Option<i16>,
    /// Erratic flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub lt_erratic: Option<i32>,
    /// Area flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub lt_area: Option<i32>,
    /// Underground flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub lt_underground: Option<i32>,
}

/// Geographic region (with GeoJSON geometry).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoRegion {
    /// Region ID.
    pub id: i32,
    /// Region text.
    #[serde(default)]
    pub lgr_revtxtd: Option<String>,
    /// Update time.
    #[serde(default)]
    pub lgr_updttime: Option<String>,
    /// Non-hierarchical flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub lgr_non_hierarchical: Option<i32>,
}
