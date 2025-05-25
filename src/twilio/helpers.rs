use serde::{Deserialize, Deserializer};

use crate::PriceType;

pub fn deserialize_opt_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer)? {
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => Ok(Some(s.trim().to_string())),
        None => Ok(None),
    }
}

pub fn deserialize_opt_usize<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        Str(String),
        Int(usize),
    }

    match Option::<StringOrInt>::deserialize(deserializer)? {
        Some(StringOrInt::Str(s)) if s.trim().is_empty() => Ok(None),
        Some(StringOrInt::Str(s)) => s.parse().map(Some).map_err(serde::de::Error::custom),
        Some(StringOrInt::Int(i)) => Ok(Some(i)),
        None => Ok(None),
    }
}

pub fn deserialize_opt_price_type<'de, D>(deserializer: D) -> Result<Option<PriceType>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        Str(String),
        PriceType(PriceType),
    }

    match Option::<StringOrNumber>::deserialize(deserializer)? {
        Some(StringOrNumber::Str(s)) if s.trim().is_empty() => Ok(None),
        Some(StringOrNumber::Str(s)) => s.parse().map(Some).map_err(serde::de::Error::custom),
        Some(StringOrNumber::PriceType(f)) => Ok(Some(f)),
        None => Ok(None),
    }
}
