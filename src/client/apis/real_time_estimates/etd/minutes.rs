use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{convert::TryFrom, str::FromStr};

pub const LEAVING_STRING: &str = "Leaving";
pub const LEAVING_NUMBER: i32 = 0;

#[derive(Debug, Clone, PartialEq)]
pub enum EtdEstimateMinutes {
    Leaving,
    Minutes(i32),
}

impl EtdEstimateMinutes {
    pub fn from_string<T: AsRef<str>>(string: T) -> Result<EtdEstimateMinutes> {
        if string.as_ref() == LEAVING_STRING {
            return Ok(EtdEstimateMinutes::Leaving);
        }

        let minutes = i32::from_str(string.as_ref());
        if let Ok(number) = minutes {
            return Ok(EtdEstimateMinutes::from_number(number));
        }

        Err(anyhow!("Does not match \"Leaving\" or a number"))
    }

    pub fn from_number(number: i32) -> EtdEstimateMinutes {
        if number <= LEAVING_NUMBER {
            return EtdEstimateMinutes::Leaving;
        }

        EtdEstimateMinutes::Minutes(number)
    }

    pub fn to_string(&self) -> String {
        match self {
            EtdEstimateMinutes::Leaving => LEAVING_STRING.to_string(),
            EtdEstimateMinutes::Minutes(number) => number.to_string(),
        }
    }

    pub fn to_number(&self) -> i32 {
        match self {
            EtdEstimateMinutes::Leaving => LEAVING_NUMBER,
            EtdEstimateMinutes::Minutes(number) => *number,
        }
    }
}

impl TryFrom<String> for EtdEstimateMinutes {
    type Error = anyhow::Error;

    fn try_from(estimate_minutes_string: String) -> std::result::Result<Self, Self::Error> {
        EtdEstimateMinutes::from_string(&estimate_minutes_string)
    }
}

impl Serialize for EtdEstimateMinutes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EtdEstimateMinutes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        EtdEstimateMinutes::try_from(s).map_err(serde::de::Error::custom)
    }
}
