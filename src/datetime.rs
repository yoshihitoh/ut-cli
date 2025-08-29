use std::convert::TryInto;
use std::fmt::Debug;
use std::str::FromStr;

use regex::Regex;
use thiserror::Error;

use crate::parse::extract_number;
use crate::validate::{validate_number, IntoValidationError};
use chrono::{ DateTime, MappedLocalTime, NaiveDate, NaiveTime, TimeZone};

#[derive(Error, Debug, PartialEq)]
pub enum YmdError {
    #[error("Wrong ymd text: '{0}'. text must be in `yyyyMMdd` or `yyyy-MM-dd` format.")]
    WrongFormat(String),

    #[error("Wrong year: '{0}'. year must be between {1} and {2}.")]
    WrongYear(String, i32, i32),

    #[error("Wrong month: '{0}'. month must be between 1 and 12.")]
    WrongMonth(String),

    #[error("Wrong day: '{0}'. day must be between 1 and 31.")]
    WrongDay(String),

    #[error("Wrong date: '{0}'.")]
    WrongDate(String),
}

impl IntoValidationError for YmdError {
    fn into_validation_error(self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Ymd {
    y: i32,
    m: u32,
    d: u32,
}

impl Ymd {
    pub fn into_datetime<Tz>(self, tz: &Tz) -> Result<DateTime<Tz>, YmdError>
    where
        Tz: TimeZone + Debug,
    {
        let date: NaiveDate = self.try_into()?;
        match tz.from_local_datetime(&date.and_time(NaiveTime::MIN)) {
            MappedLocalTime::Single(datetime) => Ok(datetime),
            MappedLocalTime::Ambiguous(a, b) => Err(YmdError::WrongDate(
                format!(
                    "Date is ambiguous. A:{:?}, B:{:?}",
                    a, b
                )
            )),
            MappedLocalTime::None => Err(YmdError::WrongDate(
                format!(
                    "Date does not exist. ymd:{:?}, tz:{:?}",
                    &self, tz
                )
            ))
        }
    }
}

impl FromStr for Ymd {
    type Err = YmdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?:(\d{4})(\d{2})(\d{2})|(\d{4})[-/](\d{1,2})[-/](\d{1,2}))$")
            .expect("wrong regex pattern");

        re.captures(s)
            .map(|capture| {
                let y = extract_number(capture.get(1).or_else(|| capture.get(4)));
                let m = extract_number(capture.get(2).or_else(|| capture.get(5)));
                let d = extract_number(capture.get(3).or_else(|| capture.get(6)));

                validate_number(y, 1900, 2999, || {
                    YmdError::WrongYear(s.to_string(), 1900, 2999)
                })
                .and_then(|_| validate_number(m, 1, 12, || YmdError::WrongMonth(s.to_string())))
                .and_then(|_| validate_number(d, 1, 31, || YmdError::WrongDay(s.to_string())))
                .map(|_| Ymd { y, m, d })
            })
            .unwrap_or_else(|| Err(YmdError::WrongFormat(s.to_string())))
    }
}

impl TryInto<NaiveDate> for Ymd {
    type Error = YmdError;

    fn try_into(self) -> Result<NaiveDate, Self::Error> {
        NaiveDate::from_ymd_opt(self.y, self.m, self.d)
            .ok_or_else(|| YmdError::WrongDate(format!("Date does not exist. {:?}", self)))
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum HmsError {
    #[error("Wrong hms text: '{0}'. text must be in `Hmmss` or `HH:mm:ss` format.")]
    WrongFormat(String),

    #[error("Wrong hour: '{0}'. hour must be between 0 and 23.")]
    WrongHour(String),

    #[error("Wrong minute: '{0}'. minute must be between 0 and 59.")]
    WrongMinute(String),

    #[error("Wrong second: '{0}'. second must be between 0 and 59.")]
    WrongSecond(String),
}

impl IntoValidationError for HmsError {
    fn into_validation_error(self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Hms {
    h: u32,
    m: u32,
    s: u32,
}

impl FromStr for Hms {
    type Err = HmsError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?:(\d{2})(\d{2})(\d{2})|(\d{1,2})[:](\d{1,2})[:](\d{1,2}))$")
            .expect("wrong regex pattern");

        re.captures(text)
            .map(|capture| {
                let h = extract_number(capture.get(1).or_else(|| capture.get(4)));
                let m = extract_number(capture.get(2).or_else(|| capture.get(5)));
                let s = extract_number(capture.get(3).or_else(|| capture.get(6)));

                validate_number(h, 0, 23, || HmsError::WrongHour(text.to_string()))
                    .and_then(|_| {
                        validate_number(m, 0, 59, || HmsError::WrongMinute(text.to_string()))
                    })
                    .and_then(|_| {
                        validate_number(s, 0, 59, || HmsError::WrongSecond(text.to_string()))
                    })
                    .map(|_| Hms { h, m, s })
            })
            .unwrap_or_else(|| Err(HmsError::WrongFormat(text.to_string())))
    }
}

impl Into<NaiveTime> for Hms {
    fn into(self) -> NaiveTime {
        NaiveTime::from_hms_opt(self.h, self.m, self.s).expect("Wrong time format")
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::datetime::{Hms, Ymd};
    use chrono::Local;

    fn ymd(y: i32, m: u32, d: u32) -> Ymd {
        Ymd { y, m, d }
    }

    fn hms(h: u32, m: u32, s: u32) -> Hms {
        Hms { h, m, s }
    }

    #[test]
    fn ymd_from_str() {
        assert_eq!(Ymd::from_str("20190621"), Ok(ymd(2019, 6, 21)),);
        assert_eq!(Ymd::from_str("2019-06-21"), Ok(ymd(2019, 6, 21)),);
        assert_eq!(Ymd::from_str("2019/06/21"), Ok(ymd(2019, 6, 21)),);
        assert_eq!(Ymd::from_str("2019/6/21"), Ok(ymd(2019, 6, 21)),);

        let r = Ymd::from_str("2020/2/29");
        assert!(r.is_ok());
        assert!(r.unwrap().into_datetime(&Local).is_ok());

        let r = Ymd::from_str("2019/2/29");
        assert!(r.is_ok());
        assert!(r.unwrap().into_datetime(&Local).is_err());
    }

    #[test]
    fn hms_from_str() {
        assert_eq!(Hms::from_str("112233"), Ok(hms(11, 22, 33)));
        assert_eq!(Hms::from_str("11:22:33"), Ok(hms(11, 22, 33)));
        assert_eq!(Hms::from_str("1:2:3"), Ok(hms(1, 2, 3)));

        assert_eq!(Hms::from_str("00:00:00"), Ok(hms(0, 0, 0)));
        assert_eq!(Hms::from_str("23:59:59"), Ok(hms(23, 59, 59)));

        assert!(Hms::from_str("").is_err());
        assert!(Hms::from_str("1122334").is_err());
        assert!(Hms::from_str("11").is_err());
        assert!(Hms::from_str("11:").is_err());
        assert!(Hms::from_str("1122").is_err());
        assert!(Hms::from_str("11:22").is_err());
        assert!(Hms::from_str("11:22:").is_err());
        assert!(Hms::from_str("::").is_err());
    }
}
