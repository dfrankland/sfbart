use anyhow::{Result, anyhow};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::convert::TryFrom;

pub const DIRECTION_CODE_NORTHBOUND: &'static str = "n";
pub const DIRECTION_CODE_SOUTHBOUND: &'static str = "s";

pub const DIRECTION_FULL_NORTHBOUND: &'static str = "North";
pub const DIRECTION_FULL_SOUTHBOUND: &'static str = "South";

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

    pub fn to_code(&self) -> &'static str {
        match self {
            Direction::Northbound => DIRECTION_CODE_NORTHBOUND,
            Direction::Southbound => DIRECTION_CODE_SOUTHBOUND,
        }
    }

    pub fn to_full(&self) -> &'static str {
        match self {
            Direction::Northbound => DIRECTION_FULL_NORTHBOUND,
            Direction::Southbound => DIRECTION_FULL_SOUTHBOUND,
        }
    }
}

impl TryFrom<String> for Direction {
    type Error = &'static str;

    fn try_from(direction_string: String) -> std::result::Result<Self, Self::Error> {
        let code = Direction::from_code(&direction_string);
        if code.is_ok() {
            return Ok(code.unwrap());
        }

        let full = Direction::from_full(&direction_string);
        if full.is_ok() {
            return Ok(full.unwrap());
        }

        Err("Does not match any direction")
    }
}

impl Serialize for Direction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.to_code())
    }
}

impl<'de> Deserialize<'de> for Direction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Direction::try_from(s).map_err(|e| serde::de::Error::custom(e))
    }
}
