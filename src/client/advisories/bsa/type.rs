use anyhow::{anyhow, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

pub const BSA_TYPE_DELAY: &str = "DELAY";
pub const BSA_TYPE_EMERGENCY: &str = "EMERGENCY";

#[derive(Debug, Clone, PartialEq)]
pub enum BsaType {
    Delay,
    Emergency,
}

impl BsaType {
    pub fn from_code<T: AsRef<str>>(code: T) -> Result<BsaType> {
        match code.as_ref() {
            BSA_TYPE_DELAY => Ok(BsaType::Delay),
            BSA_TYPE_EMERGENCY => Ok(BsaType::Emergency),
            _ => Err(anyhow!("Does not match any BSA type")),
        }
    }

    pub fn to_code(&self) -> &'static str {
        match self {
            BsaType::Delay => BSA_TYPE_DELAY,
            BsaType::Emergency => BSA_TYPE_EMERGENCY,
        }
    }
}

impl TryFrom<String> for BsaType {
    type Error = anyhow::Error;

    fn try_from(bsa_type_string: String) -> std::result::Result<Self, Self::Error> {
        BsaType::from_code(&bsa_type_string)
    }
}

impl Serialize for BsaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_code())
    }
}

impl<'de> Deserialize<'de> for BsaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        BsaType::try_from(s).map_err(serde::de::Error::custom)
    }
}
