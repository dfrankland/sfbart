use serde::{Deserialize, Deserializer};
use std::{fmt::Display, str::FromStr};

pub fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

pub fn bool_from_number_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let number = u8::from_str(&s).map_err(serde::de::Error::custom)?;
    Ok(number != 0)
}

pub fn deserialize_option<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    if let Ok(v) = T::deserialize(deserializer) {
        return Ok(Some(v));
    }
    Ok(None)
}

pub fn extract_cdata_section<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct CDATASection {
        #[serde(rename = "#cdata-section")]
        inner: String,
    }
    CDATASection::deserialize(deserializer).map(|cdata_section| cdata_section.inner)
}
