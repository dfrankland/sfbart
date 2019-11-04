use crate::client::{
    constants::{datetime::Time, station::Station as StationConstant, PUBLIC_KEY},
    serde_helpers::{bool_from_number_str, extract_cdata_section},
};
use anyhow::Result;
use reqwest;
use serde::{Deserialize, Deserializer, Serialize};
use url::Url;

pub fn extract_fill_time<'de, D>(deserializer: D) -> std::result::Result<Option<Time>, D::Error>
where
    D: Deserializer<'de>,
{
    if let Ok(time_string) = extract_cdata_section(deserializer) {
        if let Ok(time) = Time::from_short_string_without_tz(time_string) {
            return Ok(Some(time));
        }
        return Ok(None);
    }
    Ok(None)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Station {
    #[serde(rename = "@parking_flag", deserialize_with = "bool_from_number_str")]
    pub parking_flag: bool,
    #[serde(rename = "@bike_flag", deserialize_with = "bool_from_number_str")]
    pub bike_flag: bool,
    #[serde(
        rename = "@bike_station_flag",
        deserialize_with = "bool_from_number_str"
    )]
    pub bike_station_flag: bool,
    #[serde(rename = "@locker_flag", deserialize_with = "bool_from_number_str")]
    pub locker_flag: bool,
    pub name: StationConstant,
    pub abbr: StationConstant,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub entering: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub exiting: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub parking: String,
    #[serde(deserialize_with = "extract_fill_time")]
    pub fill_time: Option<Time>,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub car_share: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub lockers: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub bike_station_text: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub destinations: String,
    #[serde(deserialize_with = "extract_cdata_section")]
    pub transit_info: String,
    link: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stations {
    pub station: Station,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub legend: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationsResponse {
    pub stations: Stations,
    pub message: Message,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root {
    pub root: StationsResponse,
}

pub fn url<T: AsRef<str>>(orig: StationConstant, key: Option<T>) -> String {
    format!(
        "https://api.bart.gov/api/stn.aspx?cmd=stnaccess&orig={}&key={}&json=y&l=1",
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
async fn stnaccess() {
    let reponse = call::<&str>(StationConstant::Orinda, None).await.unwrap();
    assert_eq!(reponse.stations.station.fill_time.is_some(), true);
}
