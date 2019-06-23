use chrono::NaiveTime;
use failure::{Fail, ResultExt};
use regex::{Match, Regex};

use crate::argv::{ParseArgv, ValidateArgv};
use crate::error::{UtError, UtErrorKind};
use std::fmt::Debug;
use std::str::FromStr;

pub struct HmsArgv {}

impl Default for HmsArgv {
    fn default() -> Self {
        HmsArgv {}
    }
}

impl ParseArgv<NaiveTime> for HmsArgv {
    fn parse_argv(&self, hms: &str) -> Result<NaiveTime, UtError> {
        let hms = Hms::from_str(hms).context(UtErrorKind::WrongTime)?;
        Ok(NaiveTime::from_hms(hms.h, hms.m, hms.s))
    }
}

impl ValidateArgv for HmsArgv {
    fn validate_argv(hms: String) -> Result<(), String> {
        Hms::from_str(&hms)
            .context(UtErrorKind::WrongTime)
            .map(|_| ())
            .map_err(|e| {
                let e = UtError::from(e);
                format!(
                    "{}{}",
                    e,
                    e.cause()
                        .map_or("".to_string(), |c| format!(" cause: {:?}", c))
                )
            })
    }
}

#[derive(Fail, Debug, PartialEq)]
pub enum HmsError {
    #[fail(
        display = "Wrong hms text: '{}'. text must be in `Hmmss` or `HH:mm:ss` format.",
        _0
    )]
    WrongFormat(String),

    #[fail(display = "Wrong hour: '{}'. hour must be between 0 and 23.", _0)]
    WrongHour(String),

    #[fail(display = "Wrong minute: '{}'. minute must be between 0 and 59.", _0)]
    WrongMinute(String),

    #[fail(display = "Wrong second: '{}'. second must be between 0 and 59.", _0)]
    WrongSecond(String),
}

struct Hms {
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
                let h = extract_number(capture.get(1).or(capture.get(4)));
                let m = extract_number(capture.get(2).or(capture.get(5)));
                let s = extract_number(capture.get(3).or(capture.get(6)));

                validate_number(h, 0, 23, || HmsError::WrongHour(text.to_string()))
                    .and_then(|_| {
                        validate_number(m, 0, 59, || HmsError::WrongMinute(text.to_string()))
                    })
                    .and_then(|_| {
                        validate_number(s, 0, 59, || HmsError::WrongSecond(text.to_string()))
                    })
                    .map(|_| Hms { h, m, s })
            })
            .unwrap_or(Err(HmsError::WrongFormat(text.to_string())))
    }
}

fn extract_number<E: Debug, T: FromStr<Err = E>>(maybe_match: Option<Match>) -> T {
    maybe_match
        .map(|m| m.as_str().parse().expect("must be a number text."))
        .unwrap()
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
    use crate::argv::HmsArgv;
    use crate::argv::{ParseArgv, ValidateArgv};
    use chrono::NaiveTime;

    #[test]
    fn parse() {
        let argv = HmsArgv::default();

        assert_eq!(
            argv.parse_argv("112233").ok(),
            Some(NaiveTime::from_hms(11, 22, 33))
        );

        assert_eq!(
            argv.parse_argv("11:22:33").ok(),
            Some(NaiveTime::from_hms(11, 22, 33))
        );

        assert_eq!(
            argv.parse_argv("1:2:3").ok(),
            Some(NaiveTime::from_hms(1, 2, 3))
        );
    }

    #[test]
    fn validate() {
        assert!(HmsArgv::validate_argv("112233".to_string()).is_ok());
        assert!(HmsArgv::validate_argv("11:22:33".to_string()).is_ok());
        assert!(HmsArgv::validate_argv("1:2:3".to_string()).is_ok());
        assert!(HmsArgv::validate_argv("00:00:00".to_string()).is_ok());
        assert!(HmsArgv::validate_argv("23:59:59".to_string()).is_ok());

        assert!(HmsArgv::validate_argv("".to_string()).is_err());
        assert!(HmsArgv::validate_argv("1122334".to_string()).is_err());
        assert!(HmsArgv::validate_argv("11".to_string()).is_err());
        assert!(HmsArgv::validate_argv("1122".to_string()).is_err());
        assert!(HmsArgv::validate_argv("11:".to_string()).is_err());
        assert!(HmsArgv::validate_argv("11:22".to_string()).is_err());
        assert!(HmsArgv::validate_argv("11:22:".to_string()).is_err());
        assert!(HmsArgv::validate_argv("::".to_string()).is_err());
    }
}
