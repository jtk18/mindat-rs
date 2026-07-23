//! Crystallography types for the Mindat API: crystal classes, space groups,
//! and space-group sets.

use serde::{Deserialize, Serialize};

use super::serde_helpers::{deserialize_i32, deserialize_optional_i32};

/// A crystal class / point group (`/crystalclasses/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystalClass {
    /// Crystal class ID.
    #[serde(deserialize_with = "deserialize_i32")]
    pub id: i32,
    /// Crystal system.
    #[serde(default)]
    pub system: Option<String>,
    /// Hermann–Mauguin symbol.
    #[serde(default)]
    pub symbol: Option<String>,
    /// Class name.
    #[serde(default)]
    pub name: Option<String>,
}

/// A space group (`/spacegroups/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceGroup {
    /// Space group ID.
    #[serde(deserialize_with = "deserialize_i32")]
    pub id: i32,
    /// Crystal class ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub cclass: Option<i32>,
    /// Space group symbol/text.
    #[serde(default)]
    pub sgtext: Option<String>,
}

/// A space group setting (`/spacegroupsets/`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceGroupSet {
    /// Space group set ID.
    #[serde(deserialize_with = "deserialize_i32")]
    pub id: i32,
    /// Space group ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub spacegroup: Option<i32>,
    /// Crystal class ID.
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub cclass: Option<i32>,
    /// Space group set symbol/text.
    #[serde(default)]
    pub sgstext: Option<String>,
}

/// Builder for `/crystalclasses/` query parameters.
#[derive(Debug, Clone, Default)]
pub struct CrystalClassesQuery {
    /// Filter by crystal system.
    pub system: Option<String>,
    /// Filter by symbol.
    pub symbol: Option<String>,
    /// Filter by a set of IDs.
    pub id_in: Option<Vec<i32>>,
    /// Page number.
    pub page: Option<i32>,
    /// Page size.
    pub page_size: Option<i32>,
}

impl CrystalClassesQuery {
    /// Create a new empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by crystal system.
    pub fn system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
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

/// Builder for `/spacegroups/` query parameters.
#[derive(Debug, Clone, Default)]
pub struct SpaceGroupsQuery {
    /// Filter by crystal class ID.
    pub cclass: Option<i32>,
    /// Filter by space group text.
    pub sgtext: Option<String>,
    /// Filter by a set of IDs.
    pub id_in: Option<Vec<i32>>,
    /// Page number.
    pub page: Option<i32>,
    /// Page size.
    pub page_size: Option<i32>,
}

impl SpaceGroupsQuery {
    /// Create a new empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by crystal class ID.
    pub fn crystal_class(mut self, cclass: i32) -> Self {
        self.cclass = Some(cclass);
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
