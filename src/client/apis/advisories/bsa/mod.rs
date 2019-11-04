pub mod r#type;

use self::r#type::BsaType;
use crate::client::{
    constants::{
        datetime::{deserialize_with_tz, Date, DateTime, Time},
        PUBLIC_KEY,
    },
    serde_helpers::extract_cdata_section,
};
use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bsa {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    pub station: String,
    pub r#type: Option<BsaType>,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub description: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub sms_text: String,
    pub posted: Option<DateTime>,
    pub expires: Option<DateTime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BsaResponse {
    pub date: Date,
    #[serde(deserialize_with = "deserialize_with_tz")]
    pub time: Time,
    pub bsa: Vec<Bsa>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root {
    pub root: BsaResponse,
}

pub fn url<T: AsRef<str>>(key: Option<T>) -> String {
    format!(
        "https://api.bart.gov/api/bsa.aspx?cmd=bsa&key={}&json=y",
        key.map(|k| String::from(k.as_ref()))
            .unwrap_or_else(|| String::from(PUBLIC_KEY))
    )
}

pub async fn call<T: AsRef<str>>(key: Option<T>) -> Result<BsaResponse> {
    let root = reqwest::get(&url(key)).await?.json::<Root>().await?;
    Ok(root.root)
}

#[tokio::test]
async fn bsa() {
    let response = call::<&str>(None).await.unwrap();
    assert_eq!(response.time.time_zone.is_some(), true);
}
