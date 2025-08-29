use std::str::FromStr;

use chrono::FixedOffset;
use regex::{Captures, Regex};
use thiserror::Error;

use crate::validate::{validate_number, IntoValidationError};

#[derive(Error, Debug, PartialEq)]
pub enum OffsetError {
    #[error("Wrong hms text: '{0}'. text must be in `Hmmss` or `HH:mm:ss` format.")]
    Format(String),

    #[error("Wrong hour: '{0}'. hour must be between 0 and 23.")]
    Hour(String),

    #[error("Wrong minute: '{0}'. minute must be between 0 and 59.")]
    Minute(String),
}

#[cfg(test)]
impl OffsetError {
    pub fn is_wrong_format(&self) -> bool {
        use OffsetError::*;
        match self {
            Format(_) => true,
            _ => false,
        }
    }

    pub fn is_wrong_hour(&self) -> bool {
        use OffsetError::*;
        match self {
            Hour(_) => true,
            _ => false,
        }
    }

    pub fn is_wrong_minute(&self) -> bool {
        use OffsetError::*;
        match self {
            Minute(_) => true,
            _ => false,
        }
    }
}

impl IntoValidationError for OffsetError {
    fn into_validation_error(self) -> String {
        format!("{}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum OffsetSign {
    None,
    Plus,
    Minus,
}

impl OffsetSign {
    pub fn apply(self, value: i32) -> i32 {
        use OffsetSign::*;
        match self {
            None | Plus => value,
            Minus => -value,
        }
    }
}

impl From<&str> for OffsetSign {
    fn from(s: &str) -> Self {
        match s {
            "+" => OffsetSign::Plus,
            "-" => OffsetSign::Minus,
            _ => panic!("Wrong offset sign."),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Offset {
    sign: OffsetSign,
    h: i32,
    m: i32,
}

impl FromStr for Offset {
    type Err = OffsetError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        fn offset_from_captures(captures: Captures, text: &str) -> Result<Offset, OffsetError> {
            let sign = captures
                .get(1)
                .map(|m| OffsetSign::from(m.as_str()))
                .unwrap_or(OffsetSign::None);

            let h = captures
                .get(2)
                .or_else(|| captures.get(4))
                .map(|s| s.as_str().parse())
                .unwrap_or_else(|| Ok(0))
                .map_err(|e| {
                    OffsetError::Hour(format!("Parse error. error:{:?}, text:{}", e, text))
                })?;
            validate_number(h, 0, 23, || {
                OffsetError::Hour(format!("Wrong number. text:{}", text))
            })?;

            let m = captures
                .get(3)
                .or_else(|| captures.get(5))
                .map(|s| s.as_str().parse())
                .unwrap_or_else(|| Ok(0))
                .map_err(|e| {
                    OffsetError::Minute(format!("Parse error. error:{:?}, text:{}", e, text))
                })?;
            validate_number(m, 0, 59, || {
                OffsetError::Minute(format!("Wrong number. text:{}", text))
            })?;

            Ok(Offset { sign, h, m })
        }

        let re = Regex::new(r"^([-+])?(?:(\d{2})(\d{2})|(\d{1,2})(?:[:](\d{1,2}))?)$")
            .expect("wrong regex pattern");

        re.captures(text)
            .ok_or_else(|| OffsetError::Format(text.to_string()))
            .and_then(|captures| offset_from_captures(captures, text))
    }
}

impl From<Offset> for FixedOffset {
    fn from(val: Offset) -> Self {
        FixedOffset::east_opt(val.sign.apply(val.h * 3600 + val.m * 60))
            .expect("Wrong offset value")
    }
}

#[cfg(test)]
mod tests {
    use chrono::FixedOffset;

    use super::*;
    use crate::validate::validate_argv;

    fn offset(sign: OffsetSign, h: i32, m: i32) -> Offset {
        Offset { sign, h, m }
    }

    #[test]
    fn offset_from_str() {
        use OffsetSign::*;
        assert_eq!(Offset::from_str("0"), Ok(offset(None, 0, 0)));
        assert_eq!(Offset::from_str("+9"), Ok(offset(Plus, 9, 0)));
        assert_eq!(Offset::from_str("+09:00"), Ok(offset(Plus, 9, 0)));
        assert_eq!(Offset::from_str("-10"), Ok(offset(Minus, 10, 0)));
        assert_eq!(Offset::from_str("-10:00"), Ok(offset(Minus, 10, 0)));
        assert_eq!(Offset::from_str("00:00"), Ok(offset(None, 0, 0)));
        assert_eq!(Offset::from_str("00:00"), Ok(offset(None, 0, 0)));

        let r = Offset::from_str("");
        assert!(r.is_err());
        assert!(r.err().unwrap().is_wrong_format());

        let r = Offset::from_str("24:00");
        assert!(r.is_err());
        assert!(r.err().unwrap().is_wrong_hour());

        let r = Offset::from_str("23:60");
        assert!(r.is_err());
        assert!(r.err().unwrap().is_wrong_minute());
    }

    #[test]
    fn offset_into_fixedoffset() {
        use OffsetSign::*;
        assert_eq!(
            FixedOffset::east_opt(9 * 3600),
            Some(offset(Plus, 9, 0).into())
        );
        assert_eq!(
            FixedOffset::east_opt(-10 * 3600),
            Some(offset(Minus, 10, 0).into())
        );
        assert_eq!(
            FixedOffset::east_opt(5 * 3600 + 45 * 60),
            Some(offset(None, 5, 45).into())
        );
    }

    #[test]
    fn validate() {
        let validate_argv = |s: &str| validate_argv::<Offset, OffsetError>(s.to_string());

        assert!(validate_argv("0000").is_ok());
        assert!(validate_argv("00:00").is_ok());
        assert!(validate_argv("0:0").is_ok());
        assert!(validate_argv("0").is_ok());

        assert!(validate_argv("+0900").is_ok());
        assert!(validate_argv("+09:00").is_ok());
        assert!(validate_argv("+9:0").is_ok());
        assert!(validate_argv("+9").is_ok());

        assert!(validate_argv("+05:45").is_ok());
        assert!(validate_argv("-10:00").is_ok());

        assert!(validate_argv("").is_err());
        assert!(validate_argv("100").is_err());
        assert!(validate_argv("10300").is_err());
        assert!(validate_argv(":").is_err());
        assert!(validate_argv("24").is_err());
        assert!(validate_argv("23:60").is_err());
    }
}
