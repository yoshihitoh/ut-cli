use chrono::FixedOffset;
use failure::{Fail, ResultExt};

use crate::argv::{ParseArgv, ValidateArgv};
use crate::error::{UtError, UtErrorKind};
use regex::{Match, Regex};
use std::fmt::Debug;
use std::str::FromStr;

pub struct OffsetArgv {}

impl Default for OffsetArgv {
    fn default() -> Self {
        OffsetArgv {}
    }
}

impl ParseArgv<FixedOffset> for OffsetArgv {
    fn parse_argv(&self, s: &str) -> Result<FixedOffset, UtError> {
        Offset::from_str(s)
            .map(|o| FixedOffset::east((o.h * 3600 + o.m * 60) * o.sign))
            .context(UtErrorKind::WrongTimeOffset)
            .map_err(UtError::from)
    }
}

impl ValidateArgv for OffsetArgv {
    fn validate_argv(s: String) -> Result<(), String> {
        Offset::from_str(&s)
            .map(|_| ())
            .context(UtErrorKind::WrongTimeOffset)
            .map_err(|e| {
                let e = UtError::from(e);
                format!(
                    "{}{}",
                    e,
                    e.cause()
                        .map_or("".to_string(), |f| format!("cause: {:?}", f))
                )
            })
    }
}

#[derive(Fail, Debug, PartialEq)]
pub enum OffsetError {
    #[fail(
        display = "Wrong hms text: '{}'. text must be in `Hmmss` or `HH:mm:ss` format.",
        _0
    )]
    WrongFormat(String),

    #[fail(display = "Wrong hour: '{}'. hour must be between 0 and 23.", _0)]
    WrongHour(String),

    #[fail(display = "Wrong minute: '{}'. minute must be between 0 and 59.", _0)]
    WrongMinute(String),
}

struct Offset {
    sign: i32,
    h: i32,
    m: i32,
}

impl FromStr for Offset {
    type Err = OffsetError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^([-+])?(?:(\d{2})(\d{2})|(\d{1,2})(?:[:](\d{1,2}))?)$")
            .expect("wrong regex pattern");

        re.captures(text)
            .map(|capture| {
                let sign = capture
                    .get(1)
                    .map(|m| match m.as_str() {
                        "-" => -1,
                        _ => 1,
                    })
                    .unwrap_or(1);
                let h = extract_number(capture.get(2).or_else(|| capture.get(4))).unwrap_or(0);
                let m = extract_number(capture.get(3).or_else(|| capture.get(5))).unwrap_or(0);

                validate_number(h, 0, 23, || OffsetError::WrongHour(text.to_string()))
                    .and_then(|_| {
                        validate_number(m, 0, 59, || OffsetError::WrongMinute(text.to_string()))
                    })
                    .map(|_| Offset { sign, h, m })
            })
            .unwrap_or_else(|| Err(OffsetError::WrongFormat(text.to_string())))
    }
}

fn extract_number<E: Debug, T: FromStr<Err = E>>(maybe_match: Option<Match>) -> Option<T> {
    maybe_match.map(|m| m.as_str().parse().expect("must be a number text."))
}

fn validate_number<T: PartialOrd, E, F: Fn() -> E>(n: T, min: T, max: T, f: F) -> Result<(), E> {
    if n >= min && n <= max {
        Ok(())
    } else {
        Err(f())
    }
}

#[cfg(test)]
mod tests {
    use crate::argv::{OffsetArgv, ParseArgv, ValidateArgv};
    use chrono::FixedOffset;

    #[test]
    fn parse() {
        let argv = OffsetArgv::default();
        assert_eq!(argv.parse_argv("0").ok(), Some(FixedOffset::east(0)));
        assert_eq!(argv.parse_argv("9").ok(), Some(FixedOffset::east(9 * 3600)));
        assert_eq!(
            argv.parse_argv("-10").ok(),
            Some(FixedOffset::east(-10 * 3600))
        );

        assert_eq!(argv.parse_argv("00:00").ok(), Some(FixedOffset::east(0)));

        assert_eq!(
            argv.parse_argv("+0900").ok(),
            Some(FixedOffset::east(9 * 3600))
        );
        assert_eq!(
            argv.parse_argv("-1000").ok(),
            Some(FixedOffset::east(-10 * 3600))
        );

        assert_eq!(
            argv.parse_argv("+5:45").ok(),
            Some(FixedOffset::east(5 * 3600 + 45 * 60))
        );
        assert_eq!(
            argv.parse_argv("+9:00").ok(),
            Some(FixedOffset::east(9 * 3600))
        );
        assert_eq!(
            argv.parse_argv("-10:00").ok(),
            Some(FixedOffset::east(-10 * 3600))
        );
    }

    #[test]
    fn validate() {
        let validate_argv = |s: &str| OffsetArgv::validate_argv(s.to_string());

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
