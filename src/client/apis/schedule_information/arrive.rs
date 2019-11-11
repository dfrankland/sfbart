use crate::client::{
    constants::{
        datetime::{deserialize_without_tz, Date, Time},
        fare_type::FareType,
        station::Station,
        PUBLIC_KEY,
    },
    serde_helpers::{bool_from_number_str, from_str},
};
use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Leg {
    #[serde(rename = "@order", deserialize_with = "from_str")]
    order: i32,
    #[serde(rename = "@origin")]
    origin: Station,
    #[serde(rename = "@destination")]
    destination: Station,
    #[serde(rename = "@origTimeMin", deserialize_with = "deserialize_without_tz")]
    orig_time_min: Time,
    #[serde(rename = "@origTimeDate")]
    orig_time_date: Date,
    #[serde(rename = "@destTimeMin", deserialize_with = "deserialize_without_tz")]
    dest_time_min: Time,
    #[serde(rename = "@destTimeDate")]
    dest_time_date: Date,
    #[serde(rename = "@line")]
    line: String, // Should be an enum
    #[serde(rename = "@bikeflag", deserialize_with = "bool_from_number_str")]
    bikeflag: bool,
    // Does not always exactly match an acutal station name (e.g. "Warm Springs" instead of "Warm
    // Springs/South Fremont")
    #[serde(rename = "@trainHeadStation")]
    train_head_station: String,
    #[serde(rename = "@load", deserialize_with = "from_str")]
    load: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fare {
    #[serde(rename = "@amount")]
    amount: String,
    #[serde(rename = "@class")]
    class: FareType,
    #[serde(rename = "@name")]
    name: FareType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fares {
    #[serde(rename = "@level")]
    level: String,
    fare: Vec<Fare>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trip {
    #[serde(rename = "@origin")]
    pub origin: Station,
    #[serde(rename = "@destination")]
    pub destination: Station,
    #[serde(rename = "@origTimeMin", deserialize_with = "deserialize_without_tz")]
    pub orig_time_min: Time,
    #[serde(rename = "@origTimeDate")]
    pub orig_time_date: Date,
    #[serde(rename = "@destTimeMin", deserialize_with = "deserialize_without_tz")]
    pub dest_time_min: Time,
    #[serde(rename = "@destTimeDate")]
    pub dest_time_date: Date,
    #[serde(rename = "@tripTime")]
    pub trip_time: String,
    pub fares: Fares,
    pub leg: Vec<Leg>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub trip: Vec<Trip>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    pub date: Date,
    #[serde(deserialize_with = "deserialize_without_tz")]
    pub time: Time,
    #[serde(deserialize_with = "from_str")]
    pub before: i32,
    #[serde(deserialize_with = "from_str")]
    pub after: i32,
    pub request: Request,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub legend: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArriveResponse {
    pub origin: Station,
    pub destination: Station,
    pub schedule: Schedule,
    pub message: Message,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Root {
    pub root: ArriveResponse,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArriveTripsOptions {
    ZeroBeforeOneAfter,
    ZeroBeforeTwoAfter,
    ZeroBeforeThreeAfter,
    ZeroBeforeFourAfter,
    OneBeforeOneAfter,
    OneBeforeTwoAfter,
    OneBeforeThreeAfter,
    OneBeforeFourAfter,
    TwoBeforeOneAfter,
    TwoBeforeTwoAfter,
    TwoBeforeThreeAfter,
    TwoBeforeFourAfter,
    ThreeBeforeOneAfter,
    ThreeBeforeTwoAfter,
    ThreeBeforeThreeAfter,
    FourBeforeOneAfter,
    FourBeforeTwoAfter,
}

impl ArriveTripsOptions {
    pub fn as_a_b(&self) -> (u8, u8) {
        match self {
            ArriveTripsOptions::ZeroBeforeOneAfter => (0, 1),
            ArriveTripsOptions::ZeroBeforeTwoAfter => (0, 2),
            ArriveTripsOptions::ZeroBeforeThreeAfter => (0, 3),
            ArriveTripsOptions::ZeroBeforeFourAfter => (0, 4),
            ArriveTripsOptions::OneBeforeOneAfter => (1, 1),
            ArriveTripsOptions::OneBeforeTwoAfter => (1, 2),
            ArriveTripsOptions::OneBeforeThreeAfter => (1, 3),
            ArriveTripsOptions::OneBeforeFourAfter => (1, 4),
            ArriveTripsOptions::TwoBeforeOneAfter => (2, 1),
            ArriveTripsOptions::TwoBeforeTwoAfter => (2, 2),
            ArriveTripsOptions::TwoBeforeThreeAfter => (2, 3),
            ArriveTripsOptions::TwoBeforeFourAfter => (2, 4),
            ArriveTripsOptions::ThreeBeforeOneAfter => (3, 1),
            ArriveTripsOptions::ThreeBeforeTwoAfter => (3, 2),
            ArriveTripsOptions::ThreeBeforeThreeAfter => (3, 3),
            ArriveTripsOptions::FourBeforeOneAfter => (4, 1),
            ArriveTripsOptions::FourBeforeTwoAfter => (4, 2),
        }
    }
}

impl Default for ArriveTripsOptions {
    fn default() -> Self {
        ArriveTripsOptions::TwoBeforeTwoAfter
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArriveOptions {
    pub orig: Station,
    pub dest: Station,
    pub time: Option<NaiveTime>,
    pub date: Option<NaiveDate>,
    pub trips: Option<ArriveTripsOptions>,
}

pub fn url<T: AsRef<str>>(options: &ArriveOptions, key: Option<T>) -> String {
    let key = key
        .map(|k| String::from(k.as_ref()))
        .unwrap_or_else(|| String::from(PUBLIC_KEY));
    let orig = options.orig.to_abbr();
    let dest = options.dest.to_abbr();
    let time = options
        .time
        .map(|naive_time| naive_time.format("%I:%M+%P").to_string())
        .unwrap_or_else(|| String::from("now"));
    let date = options
        .date
        .map(|naive_date| naive_date.format("%m/%d/%Y").to_string())
        .unwrap_or_else(|| String::from("today"));
    let (a, b) = options.trips.clone().unwrap_or_default().as_a_b();
    format!(
        "https://api.bart.gov/api/sched.aspx?cmd=arrive&json=y&l=1&key={}&orig={}&dest={}&time={}&date={}&a={}&b={}",
        key,
        orig,
        dest,
        time,
        date,
        a,
        b,
    )
}

pub async fn call<T: AsRef<str>>(
    options: &ArriveOptions,
    key: Option<T>,
) -> Result<ArriveResponse> {
    let root = reqwest::get(&url(options, key))
        .await?
        .json::<Root>()
        .await?;
    Ok(root.root)
}

#[tokio::test]
async fn arrive() {
    let arrive_response = call::<&str>(
        &ArriveOptions {
            orig: Station::Orinda,
            dest: Station::Embarcadero,
            time: None,
            date: None,
            trips: None,
        },
        None,
    )
    .await
    .unwrap();
    assert_eq!(
        arrive_response.message.legend,
        "bikeflag: 1 = bikes allowed. 0 = no bikes allowed."
    )
}
