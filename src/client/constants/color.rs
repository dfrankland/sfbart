use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

pub const COLOR_YELLOW_HEX: &str = "#ffff33";
pub const COLOR_ORANGE_HEX: &str = "#ff9933";
pub const COLOR_GREEN_HEX: &str = "#339933";
pub const COLOR_RED_HEX: &str = "#ff0000";
pub const COLOR_BLUE_HEX: &str = "#0099cc";
pub const COLOR_PURPLE_HEX: &str = "#c463c5";
pub const COLOR_BEIGE_HEX: &str = "#d5cfa3";
pub const COLOR_WHITE_HEX: &str = "#ffffff";

pub const COLOR_YELLOW_NAME: &str = "YELLOW";
pub const COLOR_ORANGE_NAME: &str = "ORANGE";
pub const COLOR_GREEN_NAME: &str = "GREEN";
pub const COLOR_RED_NAME: &str = "RED";
pub const COLOR_BLUE_NAME: &str = "BLUE";
pub const COLOR_PURPLE_NAME: &str = "PURPLE";
pub const COLOR_NONE_NAME: &str = "";
pub const COLOR_BEIGE_NAME: &str = "BEIGE";
pub const COLOR_WHITE_NAME: &str = "WHITE";

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Yellow,
    Orange,
    Green,
    Red,
    Blue,
    Beige,
    Purple,
    White,
}

impl Color {
    pub fn from_code<T: AsRef<str>>(code: T) -> Result<Color> {
        match code.as_ref() {
            COLOR_YELLOW_HEX => Ok(Color::Yellow),
            COLOR_ORANGE_HEX => Ok(Color::Orange),
            COLOR_GREEN_HEX => Ok(Color::Green),
            COLOR_RED_HEX => Ok(Color::Red),
            COLOR_BLUE_HEX => Ok(Color::Blue),
            COLOR_PURPLE_HEX => Ok(Color::Purple),
            COLOR_BEIGE_HEX => Ok(Color::Beige),
            COLOR_WHITE_HEX => Ok(Color::White),
            _ => Err(anyhow!("Does not match any color")),
        }
    }

    pub fn from_full<T: AsRef<str>>(full: T) -> Result<Color> {
        match full.as_ref() {
            COLOR_YELLOW_NAME => Ok(Color::Yellow),
            COLOR_ORANGE_NAME => Ok(Color::Orange),
            COLOR_GREEN_NAME => Ok(Color::Green),
            COLOR_RED_NAME => Ok(Color::Red),
            COLOR_BLUE_NAME => Ok(Color::Blue),
            COLOR_NONE_NAME | COLOR_PURPLE_NAME => Ok(Color::Purple),
            COLOR_BEIGE_NAME => Ok(Color::Beige),
            COLOR_WHITE_NAME => Ok(Color::White),
            _ => Err(anyhow!("Does not match any color")),
        }
    }

    pub fn to_code(&self) -> &str {
        match self {
            Color::Yellow => COLOR_YELLOW_HEX,
            Color::Orange => COLOR_ORANGE_HEX,
            Color::Green => COLOR_GREEN_HEX,
            Color::Red => COLOR_RED_HEX,
            Color::Blue => COLOR_BLUE_HEX,
            Color::Purple => COLOR_PURPLE_HEX,
            Color::Beige => COLOR_BEIGE_HEX,
            Color::White => COLOR_WHITE_HEX,
        }
    }

    pub fn to_full(&self) -> &str {
        match self {
            Color::Yellow => COLOR_YELLOW_NAME,
            Color::Orange => COLOR_ORANGE_NAME,
            Color::Green => COLOR_GREEN_NAME,
            Color::Red => COLOR_RED_NAME,
            Color::Blue => COLOR_BLUE_NAME,
            Color::Purple => COLOR_PURPLE_NAME,
            Color::Beige => COLOR_BEIGE_NAME,
            Color::White => COLOR_WHITE_NAME,
        }
    }
}

impl TryFrom<String> for Color {
    type Error = anyhow::Error;

    fn try_from(color_string: String) -> std::result::Result<Self, Self::Error> {
        let code = Color::from_code(&color_string);
        if let Ok(color) = code {
            return Ok(color);
        }

        let full = Color::from_full(&color_string);
        if let Ok(color) = full {
            return Ok(color);
        }

        full
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_code())
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Color::try_from(s).map_err(serde::de::Error::custom)
    }
}
