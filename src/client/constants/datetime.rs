use anyhow::{anyhow, Result};
use chrono::{
    DateTime as ChronoDateTime, FixedOffset, LocalResult, NaiveDate, NaiveDateTime, NaiveTime,
    TimeZone as ChronoTimeZone,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{convert::TryFrom, fmt};

pub const CHRONO_DATE_FORMAT: &str = "%m/%d/%Y";
pub const CHRONO_DATEWEIRD_FORMAT: &str = "%b %e, %Y";
pub const CHRONO_TIMEFULL_FORMAT: &str = "%r";
pub const CHRONO_TIMEFULLWEIRD_FORMAT: &str = "%H:%M:%S %p";
pub const CHRONO_TIMESHORT_FORMAT: &str = "%l:%M %p";
pub const CHRONO_DATETIME_FORMAT: &str = "%a %b %d %Y %I:%M %p";

pub const CHRONO_DATE_LENGTH: usize = 10;
pub const CHRONO_DATEWEIRDMAX_LENGTH: usize = 12;
pub const CHRONO_DATEWEIRDMIN_LENGTH: usize = 11;
pub const CHRONO_TIME_LENGTH: usize = 11;
pub const CHRONO_DATETIME_LENGTH: usize = 24;

pub const TIMEZONE_PDT_STRING: &str = "PDT";
pub const TIMEZONE_PST_STRING: &str = "PST";

pub const TIMEZONE_PDT_OFFSET: u8 = 7;
pub const TIMEZONE_PST_OFFSET: u8 = 8;

const HOUR: i32 = 3600;

#[derive(Debug, Clone, PartialEq)]
pub struct DateTime {
    pub inner: ChronoDateTime<FixedOffset>,
    pub time_zone: TimeZone,
}

impl DateTime {
    pub fn from_string<T: AsRef<str>>(string: T) -> Result<DateTime> {
        let parts: Vec<&str> = string.as_ref().rsplitn(2, ' ').collect();
        if parts.len() != 2 {
            return Err(anyhow!("String is not formatted correctly with spaces"));
        }
        let time_zone = TimeZone::from_string(parts[0])?;
        let local_datetime = NaiveDateTime::parse_from_str(
            &parts[1][0..CHRONO_DATETIME_LENGTH],
            CHRONO_DATETIME_FORMAT,
        )?;
        match time_zone
            .to_fixed_offset()
            .from_local_datetime(&local_datetime)
        {
            LocalResult::Single(datetime) => Ok(DateTime {
                inner: datetime,
                time_zone,
            }),
            _ => Err(anyhow!("Date was ambigous or invalid")),
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.inner.format(CHRONO_DATETIME_FORMAT),
            self.time_zone
        )
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::from_string(s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Date(pub NaiveDate);

impl TryFrom<String> for Date {
    type Error = anyhow::Error;

    fn try_from(date_string: String) -> std::result::Result<Self, Self::Error> {
        let date_string_len = date_string.len();

        if date_string_len == CHRONO_DATEWEIRDMAX_LENGTH
            || date_string_len == CHRONO_DATEWEIRDMIN_LENGTH
        {
            if let Ok(naive_date) = NaiveDate::parse_from_str(&date_string, CHRONO_DATEWEIRD_FORMAT)
            {
                return Ok(Date(naive_date));
            }
        }

        if date_string_len >= CHRONO_DATE_LENGTH {
            if let Ok(naive_date) =
                NaiveDate::parse_from_str(&date_string[0..CHRONO_DATE_LENGTH], CHRONO_DATE_FORMAT)
            {
                return Ok(Date(naive_date));
            }
        }

        Err(anyhow!("Can't parse date"))
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self.0.format(CHRONO_DATE_FORMAT)))
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Date::try_from(s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    pub time: NaiveTime,
    pub time_zone: Option<TimeZone>,
}

impl Time {
    pub fn from_full_string_with_tz<T: AsRef<str>>(string: T) -> Result<Time> {
        let parts: Vec<&str> = string.as_ref().rsplitn(2, ' ').collect();
        if parts.len() != 2 {
            return Err(anyhow!("String is not formatted correctly with spaces"));
        }
        let time_zone = TimeZone::from_string(parts[0])?;
        let time = {
            let time_string = &parts[1][0..CHRONO_TIME_LENGTH];
            let weird = NaiveTime::parse_from_str(time_string, CHRONO_TIMEFULLWEIRD_FORMAT);
            if let Ok(time) = weird {
                time
            } else {
                NaiveTime::parse_from_str(time_string, CHRONO_TIMEFULL_FORMAT)?
            }
        };
        Ok(Time {
            time,
            time_zone: Some(time_zone),
        })
    }

    pub fn from_full_string_without_tz<T: AsRef<str>>(string: T) -> Result<Time> {
        let time = {
            let time_string = &string.as_ref()[0..CHRONO_TIME_LENGTH];
            let weird = NaiveTime::parse_from_str(time_string, CHRONO_TIMEFULLWEIRD_FORMAT);
            if let Ok(time) = weird {
                time
            } else {
                NaiveTime::parse_from_str(time_string, CHRONO_TIMEFULL_FORMAT)?
            }
        };
        Ok(Time {
            time,
            time_zone: None,
        })
    }

    pub fn from_short_string_without_tz<T: AsRef<str>>(string: T) -> Result<Time> {
        let time = NaiveTime::parse_from_str(string.as_ref(), CHRONO_TIMESHORT_FORMAT)?;
        Ok(Time {
            time,
            time_zone: None,
        })
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(time_zone) = &self.time_zone {
            write!(
                f,
                "{} {}",
                self.time.format(CHRONO_TIMEFULL_FORMAT),
                time_zone
            )
        } else {
            write!(f, "{}", self.time.format(CHRONO_TIMEFULL_FORMAT))
        }
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub fn deserialize_with_tz<'de, D>(deserializer: D) -> Result<Time, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Time::from_full_string_with_tz(s).map_err(serde::de::Error::custom)
}

pub fn deserialize_without_tz<'de, D>(deserializer: D) -> Result<Time, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if let Ok(time) = Time::from_short_string_without_tz(&s) {
        return Ok(time);
    }

    Time::from_full_string_without_tz(s).map_err(serde::de::Error::custom)
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeZone {
    Pdt,
    Pst,
}

impl TimeZone {
    pub fn from_string<T: AsRef<str>>(string: T) -> Result<TimeZone> {
        match string.as_ref() {
            TIMEZONE_PDT_STRING => Ok(TimeZone::Pdt),
            TIMEZONE_PST_STRING => Ok(TimeZone::Pst),
            _ => Err(anyhow!("Does not match any timezone")),
        }
    }

    pub fn from_number(number: u8) -> Result<TimeZone> {
        match number {
            TIMEZONE_PDT_OFFSET => Ok(TimeZone::Pdt),
            TIMEZONE_PST_OFFSET => Ok(TimeZone::Pst),
            _ => Err(anyhow!("Does not match any timezone")),
        }
    }

    pub fn to_number(&self) -> u8 {
        match self {
            TimeZone::Pdt => TIMEZONE_PDT_OFFSET,
            TimeZone::Pst => TIMEZONE_PST_OFFSET,
        }
    }

    pub fn to_fixed_offset(&self) -> FixedOffset {
        FixedOffset::west(i32::from(self.to_number()) * HOUR)
    }
}

impl fmt::Display for TimeZone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeZone::Pdt => write!(f, "{}", TIMEZONE_PDT_STRING),
            TimeZone::Pst => write!(f, "{}", TIMEZONE_PST_STRING),
        }
    }
}

impl Serialize for TimeZone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for TimeZone {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        TimeZone::from_string(s).map_err(serde::de::Error::custom)
    }
}
