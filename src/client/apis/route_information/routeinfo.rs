use crate::client::{
    constants::{color::Color, station::Station, PUBLIC_KEY},
    serde_helpers::{bool_from_number_str, from_str},
};
use anyhow::Result;
use chrono::NaiveDate;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteConfig {
    pub station: Vec<Station>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Route {
    pub name: String, // Should be an enum
    pub abbr: String, // Should be an enum
    #[serde(rename = "routeID")]
    pub route_id: String, // Should be an enum
    #[serde(deserialize_with = "from_str")]
    pub number: i32,
    pub origin: Station,
    pub destination: Station,
    pub hexcolor: Color,
    pub color: Color,
    #[serde(deserialize_with = "bool_from_number_str")]
    pub holidays: bool,
    #[serde(deserialize_with = "from_str")]
    pub num_stns: i32,
    pub config: RouteConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Routes {
    pub route: Route,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteInfoResponse {
    #[serde(deserialize_with = "from_str")]
    pub sched_num: i32,
    pub routes: Routes,
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root {
    pub root: RouteInfoResponse,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RouteInfoOptionsDate {
    Today,
    Date(NaiveDate),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RouteInfoOptions {
    Schedule(u8),
    Date(RouteInfoOptionsDate),
}

const URL_ROOT: &str = "https://api.bart.gov/api/route.aspx?cmd=routeinfo&json=y";
// `route` argument should be an enum
pub fn url<T: AsRef<str>>(route: u8, options: &Option<RouteInfoOptions>, key: Option<T>) -> String {
    let url_with_route_and_key = format!(
        "{}&route={}&key={}",
        URL_ROOT,
        route,
        key.map(|k| String::from(k.as_ref()))
            .unwrap_or_else(|| String::from(PUBLIC_KEY))
    );

    if let Some(route_options) = options {
        return match route_options {
            RouteInfoOptions::Schedule(schedule) => {
                format!("{}&sched={}", url_with_route_and_key, schedule)
            }
            RouteInfoOptions::Date(route_options_date) => {
                let date_param = match route_options_date {
                    RouteInfoOptionsDate::Today => String::from("today"),
                    RouteInfoOptionsDate::Date(date) => date.format("%m/%d/%Y").to_string(),
                };
                format!("{}&date={}", url_with_route_and_key, date_param)
            }
        };
    }

    url_with_route_and_key
}

// `route` argument should be an enum
pub async fn call<T: AsRef<str>>(
    route: u8,
    options: &Option<RouteInfoOptions>,
    key: Option<T>,
) -> Result<RouteInfoResponse> {
    let root = reqwest::get(&url(route, options, key))
        .await?
        .json::<Root>()
        .await?;
    Ok(root.root)
}

#[tokio::test]
async fn routeinfo() {
    let response = call::<&str>(1, &None, None).await.unwrap();
    assert_eq!(response.routes.route.abbr, "ANTC-SFIA");
}
