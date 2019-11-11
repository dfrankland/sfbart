use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

pub const FARE_TYPE_CODE_CLIPPER: &str = "clipper";
pub const FARE_TYPE_CODE_CASH: &str = "cash";
pub const FARE_TYPE_CODE_RTCCLIPPER: &str = "rtcclipper";
pub const FARE_TYPE_CODE_STUDENT: &str = "student";

pub const FARE_TYPE_FULL_CLIPPER: &str = "Clipper";
pub const FARE_TYPE_FULL_CASH: &str = "BART Blue Ticket";
pub const FARE_TYPE_FULL_RTCCLIPPER: &str = "Senior/Disabled Clipper";
pub const FARE_TYPE_FULL_STUDENT: &str = "Youth Clipper";

#[derive(Debug, Clone, PartialEq)]
pub enum FareType {
    Clipper,
    Cash,
    RtcClipper,
    Student,
}

impl FareType {
    pub fn from_code<T: AsRef<str>>(code: T) -> Result<FareType> {
        match code.as_ref() {
            FARE_TYPE_CODE_CLIPPER => Ok(FareType::Clipper),
            FARE_TYPE_CODE_CASH => Ok(FareType::Cash),
            FARE_TYPE_CODE_RTCCLIPPER => Ok(FareType::RtcClipper),
            FARE_TYPE_CODE_STUDENT => Ok(FareType::Student),
            _ => Err(anyhow!("Does not match any fare type")),
        }
    }

    pub fn from_full<T: AsRef<str>>(full: T) -> Result<FareType> {
        match full.as_ref() {
            FARE_TYPE_FULL_CLIPPER => Ok(FareType::Clipper),
            FARE_TYPE_FULL_CASH => Ok(FareType::Cash),
            FARE_TYPE_FULL_RTCCLIPPER => Ok(FareType::RtcClipper),
            FARE_TYPE_FULL_STUDENT => Ok(FareType::Student),
            _ => Err(anyhow!("Does not match fare type")),
        }
    }

    pub fn to_code(&self) -> &str {
        match self {
            FareType::Clipper => FARE_TYPE_CODE_CLIPPER,
            FareType::Cash => FARE_TYPE_CODE_CASH,
            FareType::RtcClipper => FARE_TYPE_CODE_RTCCLIPPER,
            FareType::Student => FARE_TYPE_CODE_STUDENT,
        }
    }

    pub fn to_full(&self) -> &str {
        match self {
            FareType::Clipper => FARE_TYPE_FULL_CLIPPER,
            FareType::Cash => FARE_TYPE_FULL_CASH,
            FareType::RtcClipper => FARE_TYPE_FULL_RTCCLIPPER,
            FareType::Student => FARE_TYPE_FULL_STUDENT,
        }
    }
}

impl TryFrom<String> for FareType {
    type Error = anyhow::Error;

    fn try_from(fare_type_string: String) -> std::result::Result<Self, Self::Error> {
        let code = FareType::from_code(&fare_type_string);
        if let Ok(fare_type) = code {
            return Ok(fare_type);
        }

        let full = FareType::from_full(&fare_type_string);
        if let Ok(fare_type) = full {
            return Ok(fare_type);
        }

        full
    }
}

impl Serialize for FareType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_code())
    }
}

impl<'de> Deserialize<'de> for FareType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FareType::try_from(s).map_err(serde::de::Error::custom)
    }
}
