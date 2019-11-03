pub mod minutes;

use self::minutes::EtdEstimateMinutes;
use crate::client::{
    constants::{color::Color, direction::Direction, station::Station, PUBLIC_KEY},
    serde_helpers::{from_str, bool_from_number_str},
};
use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EtdEstimate {
    minutes: EtdEstimateMinutes,
    #[serde(deserialize_with = "from_str")]
    platform: i32,
    direction: Direction,
    #[serde(deserialize_with = "from_str")]
    length: i32,
    color: Color,
    hexcolor: Color,
    #[serde(deserialize_with = "bool_from_number_str")]
    bikeflag: bool,
    #[serde(deserialize_with = "from_str")]
    delay: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Etd {
    // Does not always exactly match an acutal station name (e.g. "Warm Springs" instead of "Warm
    // Springs/South Fremont")
    pub destination: String,
    pub abbreviation: Station,
    #[serde(deserialize_with = "bool_from_number_str")]
    pub limited: bool,
    pub estimate: Vec<EtdEstimate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EtdStation {
    // Does not always exactly match an acutal station name (e.g. "Warm Springs" instead of "Warm
    // Springs/South Fremont")
    pub name: String,
    pub abbr: Station,
    pub etd: Vec<Etd>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EtdResponse {
    pub date: String, // Should be chrono NaiveDate
    pub time: String, // Should be chrono NaiveTime
    pub station: Vec<EtdStation>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root {
    pub root: EtdResponse,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EtdOptionsPlatform {
    One,
    Two,
    Three,
    Four,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EtdOptionsDirectionOrPlatform {
    Direction(Direction),
    Platform(EtdOptionsPlatform),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EtdOptions {
    OriginAll,
    OriginAndDirectionOrPlatform(Station, EtdOptionsDirectionOrPlatform),
}

const URL_ROOT: &str = "https://api.bart.gov/api/etd.aspx?cmd=etd&json=y";
pub fn url<T: AsRef<str>>(options: &EtdOptions, key: Option<T>) -> String {
    let url_with_key = format!(
        "{}&key={}",
        URL_ROOT,
        key.map(|k| String::from(k.as_ref()))
            .unwrap_or_else(|| String::from(PUBLIC_KEY))
    );
    match options {
        EtdOptions::OriginAll => format!("{}&orig=ALL", url_with_key),
        EtdOptions::OriginAndDirectionOrPlatform(station, direction_or_platform) => {
            let url_with_key_and_orig = format!("{}&orig={}", url_with_key, station.to_abbr());
            match direction_or_platform {
                EtdOptionsDirectionOrPlatform::Direction(direction) => {
                    format!("{}&dir={}", url_with_key_and_orig, direction.to_code())
                }
                EtdOptionsDirectionOrPlatform::Platform(platform) => {
                    let platform_number = match platform {
                        EtdOptionsPlatform::One => 1,
                        EtdOptionsPlatform::Two => 2,
                        EtdOptionsPlatform::Three => 3,
                        EtdOptionsPlatform::Four => 4,
                    };
                    format!("{}&plat={}", url_with_key_and_orig, platform_number)
                }
            }
        }
    }
}

pub async fn call<T: AsRef<str>>(options: &EtdOptions, key: Option<T>) -> Result<EtdResponse> {
    let root = reqwest::get(&url(options, key))
        .await?
        .json::<Root>()
        .await?;
    Ok(root.root)
}

#[tokio::test]
async fn etd() {
    let etd_response = call::<&str>(&EtdOptions::OriginAll, None).await.unwrap();
    assert_eq!(
        etd_response.message,
        "Direction not supported for ALL ETD messages."
    )
}
