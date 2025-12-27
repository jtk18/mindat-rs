//! Serde helper functions for handling API quirks.
//!
//! The Mindat API sometimes returns empty strings "" instead of null for
//! optional numeric fields. These helpers handle that gracefully.

use serde::{Deserialize, Deserializer};

/// Deserialize an optional f64 that might be an empty string.
pub fn deserialize_optional_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(f64),
        Null,
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) if s.is_empty() => Ok(None),
        StringOrNumber::String(s) => Ok(s.parse().ok()),
        StringOrNumber::Number(n) => Ok(Some(n)),
        StringOrNumber::Null => Ok(None),
    }
}

/// Deserialize an optional f32 that might be an empty string.
pub fn deserialize_optional_f32<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(f64),
        Null,
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) if s.is_empty() => Ok(None),
        StringOrNumber::String(s) => Ok(s.parse().ok()),
        StringOrNumber::Number(n) => Ok(Some(n as f32)),
        StringOrNumber::Null => Ok(None),
    }
}

/// Deserialize an optional i32 that might be an empty string.
pub fn deserialize_optional_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(i64),
        Null,
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) if s.is_empty() => Ok(None),
        StringOrNumber::String(s) => Ok(s.parse().ok()),
        StringOrNumber::Number(n) => Ok(Some(n as i32)),
        StringOrNumber::Null => Ok(None),
    }
}

/// Deserialize an optional `Vec<String>` that might be an empty string.
pub fn deserialize_optional_vec_string<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<String>),
        Null,
    }

    match StringOrVec::deserialize(deserializer)? {
        StringOrVec::String(s) if s.is_empty() => Ok(None),
        StringOrVec::String(s) => Ok(Some(vec![s])),
        StringOrVec::Vec(v) => Ok(Some(v)),
        StringOrVec::Null => Ok(None),
    }
}

/// Deserialize an optional `Vec<i32>` that might be an empty string.
pub fn deserialize_optional_vec_i32<'de, D>(deserializer: D) -> Result<Option<Vec<i32>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        String(String),
        Vec(Vec<i32>),
        Null,
    }

    match StringOrVec::deserialize(deserializer)? {
        StringOrVec::String(s) if s.is_empty() => Ok(None),
        StringOrVec::String(_) => Ok(None), // Can't parse a string as Vec<i32>
        StringOrVec::Vec(v) => Ok(Some(v)),
        StringOrVec::Null => Ok(None),
    }
}

/// Deserialize an optional u32 that might be an empty string.
pub fn deserialize_optional_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(u64),
        Null,
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) if s.is_empty() => Ok(None),
        StringOrNumber::String(s) => Ok(s.parse().ok()),
        StringOrNumber::Number(n) => Ok(Some(n as u32)),
        StringOrNumber::Null => Ok(None),
    }
}

/// Deserialize an optional i16 that might be an empty string.
pub fn deserialize_optional_i16<'de, D>(deserializer: D) -> Result<Option<i16>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(i64),
        Null,
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) if s.is_empty() => Ok(None),
        StringOrNumber::String(s) => Ok(s.parse().ok()),
        StringOrNumber::Number(n) => Ok(Some(n as i16)),
        StringOrNumber::Null => Ok(None),
    }
}

/// Deserialize an optional Vec of any deserializable type that might be an empty string.
pub fn deserialize_optional_vec<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec<T> {
        String(String),
        Vec(Vec<T>),
        Null,
    }

    match StringOrVec::deserialize(deserializer)? {
        StringOrVec::String(s) if s.is_empty() => Ok(None),
        StringOrVec::String(_) => Ok(None),
        StringOrVec::Vec(v) => Ok(Some(v)),
        StringOrVec::Null => Ok(None),
    }
}
