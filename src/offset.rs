use std::str::FromStr;

use chrono::FixedOffset;
use regex::{Captures, Regex};
use thiserror::Error;

use crate::validate::{validate_number, IntoValidationError};

#[derive(Error, Debug, PartialEq)]
pub enum OffsetError {
    #[error("Wrong hms text: '{0}'. text must be in `Hmmss` or `HH:mm:ss` format.")]
    WrongFormat(String),

    #[error("Wrong hour: '{0}'. hour must be between 0 and 23.")]
    WrongHour(String),

    #[error("Wrong minute: '{0}'. minute must be between 0 and 59.")]
    WrongMinute(String),
}

#[cfg(test)]
impl OffsetError {
    pub fn is_wrong_format(&self) -> bool {
        use OffsetError::*;
        match self {
            WrongFormat(_) => true,
            _ => false,
        }
    }

    pub fn is_wrong_hour(&self) -> bool {
        use OffsetError::*;
        match self {
            WrongHour(_) => true,
            _ => false,
        }
    }

    pub fn is_wrong_minute(&self) -> bool {
        use OffsetError::*;
        match self {
            WrongMinute(_) => true,
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
                    OffsetError::WrongHour(format!("Parse error. error:{:?}, text:{}", e, text))
                })?;
            validate_number(h, 0, 23, || {
                OffsetError::WrongHour(format!("Wrong number. text:{}", text))
            })?;

            let m = captures
                .get(3)
                .or_else(|| captures.get(5))
                .map(|s| s.as_str().parse())
                .unwrap_or_else(|| Ok(0))
                .map_err(|e| {
                    OffsetError::WrongMinute(format!("Parse error. error:{:?}, text:{}", e, text))
                })?;
            validate_number(m, 0, 59, || {
                OffsetError::WrongMinute(format!("Wrong number. text:{}", text))
            })?;

            Ok(Offset { sign, h, m })
        }

        let re = Regex::new(r"^([-+])?(?:(\d{2})(\d{2})|(\d{1,2})(?:[:](\d{1,2}))?)$")
            .expect("wrong regex pattern");

        re.captures(text)
            .ok_or_else(|| OffsetError::WrongFormat(text.to_string()))
            .and_then(|captures| offset_from_captures(captures, text))
    }
}

impl Into<FixedOffset> for Offset {
    fn into(self) -> FixedOffset {
        FixedOffset::east(self.sign.apply(self.h * 3600 + self.m * 60))
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
        assert_eq!(Offset::from_str("-10"), Ok(offset(Minus, 10, 0)));
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
        assert_eq!(FixedOffset::east(9 * 3600), offset(Plus, 9, 0).into());
        assert_eq!(FixedOffset::east(-10 * 3600), offset(Minus, 10, 0).into());
        assert_eq!(
            FixedOffset::east(5 * 3600 + 45 * 60),
            offset(None, 5, 45).into()
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
