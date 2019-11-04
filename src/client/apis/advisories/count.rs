use crate::client::{constants::{PUBLIC_KEY, datetime::{Date, Time, deserialize_with_tz}}, serde_helpers::from_str};
use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Count {
    pub date: Date,
    #[serde(deserialize_with = "deserialize_with_tz")]
    pub time: Time,
    #[serde(deserialize_with = "from_str")]
    pub traincount: i32,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root {
    pub root: Count,
}

pub fn url<T: AsRef<str>>(key: Option<T>) -> String {
    format!(
        "https://api.bart.gov/api/bsa.aspx?cmd=count&key={}&json=y",
        key.map(|k| String::from(k.as_ref()))
            .unwrap_or_else(|| String::from(PUBLIC_KEY))
    )
}

pub async fn call<T: AsRef<str>>(key: Option<T>) -> Result<Count> {
    let root = reqwest::get(&url(key)).await?.json::<Root>().await?;
    Ok(root.root)
}

#[tokio::test]
async fn count() {
    let reponse = call::<&str>(None).await.unwrap();
    assert!(reponse.traincount >= 0);
}
