//! Country types for the Mindat API.

use serde::{Deserialize, Serialize};

/// Country information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    /// Unique identifier.
    pub id: i32,
    /// Country name.
    pub text: String,
    /// Continent code (2 characters).
    pub continent: String,
    /// ISO country code (2 characters).
    pub iso: String,
    /// Latitude direction (N/S).
    pub latdir: String,
    /// Longitude direction (E/W).
    pub longdir: String,
}
