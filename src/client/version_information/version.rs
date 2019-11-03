use crate::client::constants::PUBLIC_KEY;
use serde::{Serialize, Deserialize};
use reqwest;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub api_version: String,
    pub copyright: String,
    pub license: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root {
    pub root: Version,
}

pub fn url<T: AsRef<str>>(key: Option<T>) -> String {
    format!("https://api.bart.gov/api/version.aspx?cmd=stns&key={}&json=y", key.map(|k| String::from(k.as_ref())).unwrap_or_else(|| String::from(PUBLIC_KEY)))
}

pub async fn call<T: AsRef<str>>(key: Option<T>) -> Result<Version> {
    let root = reqwest::get(&url(key)).await?.json::<Root>().await?;
    Ok(root.root)
}

#[tokio::test]
async fn version() {
    assert_eq!(call::<&str>(None).await.unwrap(), Version {
        api_version: String::from("3.10"),
        copyright: String::from("Copyright 2019 Bay Area Rapid Transit District"),
        license: String::from("http://www.bart.gov/schedules/developers/developer-license-agreement"),
        message: String::from(""),
    })
}
