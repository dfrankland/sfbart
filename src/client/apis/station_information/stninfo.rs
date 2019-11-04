use crate::client::{
    constants::{station::Station as StationConstant, PUBLIC_KEY},
    serde_helpers::extract_cdata_section,
};
use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Routes {
    pub route: Vec<String>, // Should be an enum of Routes
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Platforms {
    pub platform: Vec<String>, // Should be an enum of Platform
}

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
    pub north_routes: Routes,
    pub south_routes: Routes,
    pub north_platforms: Platforms,
    pub south_platforms: Platforms,
    pub platform_info: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub intro: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub cross_street: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub food: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub shopping: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub attraction: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    link: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stations {
    pub station: Station,
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

pub fn url<T: AsRef<str>>(orig: StationConstant, key: Option<T>) -> String {
    format!(
        "https://api.bart.gov/api/stn.aspx?cmd=stninfo&orig={}&key={}&json=y",
        orig.to_abbr(),
        key.map(|k| String::from(k.as_ref()))
            .unwrap_or_else(|| String::from(PUBLIC_KEY))
    )
}

pub async fn call<T: AsRef<str>>(
    orig: StationConstant,
    key: Option<T>,
) -> Result<StationsResponse> {
    let root = reqwest::get(&url(orig, key)).await?.json::<Root>().await?;
    Ok(root.root)
}

#[tokio::test]
async fn stninfo() {
    let reponse = call::<&str>(StationConstant::MacArthur, None)
        .await
        .unwrap();
    assert_eq!(reponse.stations.station.north_platforms.platform[0], "1");
}
