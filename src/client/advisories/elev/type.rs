use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

pub const ELEV_TYPE_ELEVATOR: &str = "ELEVATOR";

#[derive(Debug, Clone, PartialEq)]
pub enum ElevType {
    Elevator,
}

impl ElevType {
    pub fn from_code<T: AsRef<str>>(code: T) -> Result<ElevType> {
        match code.as_ref() {
            ELEV_TYPE_ELEVATOR => Ok(ElevType::Elevator),
            _ => Err(anyhow!("Does not match any elevator type")),
        }
    }

    pub fn to_code(&self) -> &'static str {
        match self {
            ElevType::Elevator => ELEV_TYPE_ELEVATOR,
        }
    }
}

impl TryFrom<String> for ElevType {
    type Error = anyhow::Error;

    fn try_from(elev_type_string: String) -> std::result::Result<Self, Self::Error> {
        ElevType::from_code(&elev_type_string)
    }
}

impl Serialize for ElevType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_code())
    }
}

impl<'de> Deserialize<'de> for ElevType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ElevType::try_from(s).map_err(serde::de::Error::custom)
    }
}
