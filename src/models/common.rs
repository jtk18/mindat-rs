//! Common types shared across multiple API endpoints.

use serde::{Deserialize, Serialize};

use super::serde_helpers::deserialize_optional_i32;

/// Relation between geomaterials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    /// ID of the related mineral.
    pub mineral_id: i32,
    /// Type of relation (1-13).
    /// 1=Synonym, 2=Mixture, 4=Structurally related, 5=Associated at type locality,
    /// 6=Epitaxial, 7=Polymorph, 8=Isostructural, 9=Chemically related,
    /// 10=Common Associates, 11=Essential minerals, 12=Common ore minerals, 13=Accessory minerals
    pub relation_type: i32,
    /// Human-readable relation type description.
    pub relation_type_text: String,
}

/// Statistics for a mineral.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinStats {
    /// Number of photos.
    pub ms_photos: i32,
    /// Number of locality entries.
    pub ms_locentries: i32,
    /// Number of votes for photos.
    pub ms_photovotes: i32,
}

/// GeoJSON Feature type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonFeature<P> {
    /// Feature type (always "Feature").
    #[serde(rename = "type")]
    pub feature_type: String,
    /// Feature ID.
    pub id: Option<i32>,
    /// Geometry data.
    pub geometry: Option<GeoJsonGeometry>,
    /// Feature properties.
    pub properties: P,
}

/// GeoJSON Geometry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GeoJsonGeometry {
    Point { coordinates: Vec<f64> },
    LineString { coordinates: Vec<Vec<f64>> },
    Polygon { coordinates: Vec<Vec<Vec<f64>>> },
}

/// GeoJSON FeatureCollection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonFeatureCollection<P> {
    /// Collection type (always "FeatureCollection").
    #[serde(rename = "type")]
    pub collection_type: String,
    /// Features in the collection.
    pub features: Vec<GeoJsonFeature<P>>,
}

/// Geographic region properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoRegionProperties {
    /// Region text description.
    pub lgr_revtxtd: Option<String>,
    /// Last update time.
    pub lgr_updttime: Option<String>,
    /// Non-hierarchical flag.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub lgr_non_hierarchical: Option<i32>,
}
