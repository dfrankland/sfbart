use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

pub const DIRECTION_CODE_NORTHBOUND: &str = "n";
pub const DIRECTION_CODE_SOUTHBOUND: &str = "s";

pub const DIRECTION_FULL_NORTHBOUND: &str = "North";
pub const DIRECTION_FULL_SOUTHBOUND: &str = "South";

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Northbound,
    Southbound,
}

impl Direction {
    pub fn from_code<T: AsRef<str>>(code: T) -> Result<Direction> {
        match code.as_ref() {
            DIRECTION_CODE_NORTHBOUND => Ok(Direction::Northbound),
            DIRECTION_CODE_SOUTHBOUND => Ok(Direction::Southbound),
            _ => Err(anyhow!("Does not match any direction")),
        }
    }

    pub fn from_full<T: AsRef<str>>(full: T) -> Result<Direction> {
        match full.as_ref() {
            DIRECTION_FULL_NORTHBOUND => Ok(Direction::Northbound),
            DIRECTION_FULL_SOUTHBOUND => Ok(Direction::Southbound),
            _ => Err(anyhow!("Does not match any direction")),
        }
    }

    pub fn to_code(&self) -> &str {
        match self {
            Direction::Northbound => DIRECTION_CODE_NORTHBOUND,
            Direction::Southbound => DIRECTION_CODE_SOUTHBOUND,
        }
    }

    pub fn to_full(&self) -> &str {
        match self {
            Direction::Northbound => DIRECTION_FULL_NORTHBOUND,
            Direction::Southbound => DIRECTION_FULL_SOUTHBOUND,
        }
    }
}

impl TryFrom<String> for Direction {
    type Error = anyhow::Error;

    fn try_from(direction_string: String) -> std::result::Result<Self, Self::Error> {
        let code = Direction::from_code(&direction_string);
        if let Ok(direction) = code {
            return Ok(direction);
        }

        let full = Direction::from_full(&direction_string);
        if let Ok(direction) = full {
            return Ok(direction);
        }

        full
    }
}

impl Serialize for Direction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_code())
    }
}

impl<'de> Deserialize<'de> for Direction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Direction::try_from(s).map_err(serde::de::Error::custom)
    }
}
