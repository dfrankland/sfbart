pub mod r#type;

use self::r#type::ElevType;
use crate::client::{constants::{PUBLIC_KEY, datetime::{Date, Time, DateTime, deserialize_with_tz}}, serde_helpers::{extract_cdata_section, deserialize_option}};
use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Elev {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    pub station: String,
    pub r#type: Option<ElevType>,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub description: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub sms_text: String,
    #[serde(deserialize_with = "deserialize_option")]
    pub posted: Option<DateTime>,
    #[serde(deserialize_with = "deserialize_option")]
    pub expires: Option<DateTime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElevResponse {
    pub date: Date,
    #[serde(deserialize_with = "deserialize_with_tz")]
    pub time: Time,
    pub bsa: Vec<Elev>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root {
    pub root: ElevResponse,
}

pub fn url<T: AsRef<str>>(key: Option<T>) -> String {
    format!(
        "https://api.bart.gov/api/bsa.aspx?cmd=elev&key={}&json=y",
        key.map(|k| String::from(k.as_ref()))
            .unwrap_or_else(|| String::from(PUBLIC_KEY))
    )
}

pub async fn call<T: AsRef<str>>(key: Option<T>) -> Result<ElevResponse> {
    let root = reqwest::get(&url(key)).await?.json::<Root>().await?;
    Ok(root.root)
}

#[tokio::test]
async fn elev() {
    let reponse = call::<&str>(None).await.unwrap();
    assert_eq!(reponse.time.time_zone.is_some(), true);
}
