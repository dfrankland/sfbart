use crate::client::constants::{station::Station as StationConstant, PUBLIC_KEY};
use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Station {
    pub name: String,
    pub abbr: StationConstant,
    pub gtfs_latitude: String,
    pub gtfs_longitude: String,
    pub address: String,
    pub city: String,
    pub county: String,
    pub state: String,
    pub zipcode: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stations {
    pub station: Vec<Station>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationsResponse {
    pub stations: Stations,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root {
    pub root: StationsResponse,
}

pub fn url<T: AsRef<str>>(key: Option<T>) -> String {
    format!(
        "https://api.bart.gov/api/stn.aspx?cmd=stns&key={}&json=y",
        key.map(|k| String::from(k.as_ref()))
            .unwrap_or_else(|| String::from(PUBLIC_KEY))
    )
}

pub async fn call<T: AsRef<str>>(key: Option<T>) -> Result<StationsResponse> {
    let root = reqwest::get(&url(key)).await?.json::<Root>().await?;
    Ok(root.root)
}

#[tokio::test]
async fn stns() {
    let reponse = call::<&str>(None).await.unwrap();
    assert_eq!(
        reponse.stations.station[0].abbr,
        StationConstant::OaklandCityCenter12thSt
    );
}
