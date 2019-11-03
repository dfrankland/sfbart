pub mod r#type;

use self::r#type::BsaType;
use crate::client::constants::PUBLIC_KEY;
use anyhow::Result;
use reqwest;
use serde::{Deserialize, Deserializer, Serialize};

fn extract_cdata_section<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
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
    pub posted: Option<String>,  // Should be a chrono DateTime
    pub expires: Option<String>, // Should be a chrono DateTime
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BsaResponse {
    pub date: String, // Should be a chrono NaiveDate
    pub time: String, // Should be a chrono NaiveTime
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
async fn version() {
    let reponse = call::<&str>(None).await.unwrap();
    assert_eq!(reponse.time.is_empty(), false);
}
