use crate::client::{
    constants::{color::Color, PUBLIC_KEY},
    serde_helpers::from_str,
};
use anyhow::Result;
use chrono::NaiveDate;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Route {
    pub name: String, // Should be an enum
    pub abbr: String, // Should be an enum
    #[serde(rename = "routeID")]
    pub route_id: String, // Should be an enum
    #[serde(deserialize_with = "from_str")]
    pub number: i32,
    pub hexcolor: Color,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Routes {
    pub route: Vec<Route>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutesResponse {
    #[serde(deserialize_with = "from_str")]
    pub sched_num: i32,
    pub routes: Routes,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root {
    pub root: RoutesResponse,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoutesOptionsDate {
    Today,
    Date(NaiveDate),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoutesOptions {
    Schedule(u8),
    Date(RoutesOptionsDate),
}

const URL_ROOT: &str = "https://api.bart.gov/api/route.aspx?cmd=routes&json=y";
pub fn url<T: AsRef<str>>(options: &Option<RoutesOptions>, key: Option<T>) -> String {
    let url_with_key = format!(
        "{}&key={}",
        URL_ROOT,
        key.map(|k| String::from(k.as_ref()))
            .unwrap_or_else(|| String::from(PUBLIC_KEY))
    );

    if let Some(route_options) = options {
        return match route_options {
            RoutesOptions::Schedule(schedule) => format!("{}&sched={}", url_with_key, schedule),
            RoutesOptions::Date(route_options_date) => {
                let date_param = match route_options_date {
                    RoutesOptionsDate::Today => String::from("today"),
                    RoutesOptionsDate::Date(date) => date.format("%m/%d/%Y").to_string(),
                };
                format!("{}&date={}", url_with_key, date_param)
            }
        };
    }

    url_with_key
}

pub async fn call<T: AsRef<str>>(
    options: &Option<RoutesOptions>,
    key: Option<T>,
) -> Result<RoutesResponse> {
    let root = reqwest::get(&url(options, key))
        .await?
        .json::<Root>()
        .await?;
    Ok(root.root)
}

#[tokio::test]
async fn routes() {
    let reponse = call::<&str>(&None, None).await.unwrap();
    assert_eq!(reponse.routes.route[0].abbr, "ANTC-SFIA");
}
