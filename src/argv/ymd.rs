use std::fmt::Debug;
use std::str::FromStr;

use chrono::{Date, LocalResult, TimeZone};
use failure::{Fail, ResultExt};
use regex::{Match, Regex};

use crate::argv::{ParseArgv, ValidateArgv};
use crate::error::{UtError, UtErrorKind};

pub struct YmdArgv<Tz: TimeZone> {
    tz: Tz,
}

impl<Tz: TimeZone> From<Tz> for YmdArgv<Tz> {
    fn from(tz: Tz) -> Self {
        YmdArgv { tz }
    }
}

impl<Tz: TimeZone> ParseArgv<Date<Tz>> for YmdArgv<Tz> {
    fn parse_argv(&self, s: &str) -> Result<Date<Tz>, UtError> {
        let ymd = Ymd::from_str(s).context(UtErrorKind::WrongDate)?;
        match self.tz.ymd_opt(ymd.y, ymd.m, ymd.d) {
            LocalResult::Single(date) => Ok(date),
            LocalResult::None => {
                Err(YmdError::WrongDate(s.to_string())).context(UtErrorKind::WrongDate)?
            }
            LocalResult::Ambiguous(_, _) => {
                Err(YmdError::WrongDate(s.to_string())).context(UtErrorKind::AmbiguousDate)?
            }
        }
    }
}

impl<Tz: TimeZone> ValidateArgv for YmdArgv<Tz> {
    fn validate_argv(s: String) -> Result<(), String> {
        Ymd::from_str(&s)
            .context(UtErrorKind::WrongDate)
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

struct Ymd {
    y: i32,
    m: u32,
    d: u32,
}

#[derive(Fail, Debug, PartialEq)]
pub enum YmdError {
    #[fail(
        display = "Wrong ymd text: '{}'. text must be in `yyyyMMdd` or `yyyy-MM-dd` format.",
        _0
    )]
    WrongFormat(String),

    #[fail(
        display = "Wrong year: '{}'. year must be between {} and {}.",
        _0, _1, _2
    )]
    WrongYear(String, i32, i32),

    #[fail(display = "Wrong month: '{}'. month must be between 1 and 12.", _0)]
    WrongMonth(String),

    #[fail(display = "Wrong day: '{}'. day must be between 1 and 31.", _0)]
    WrongDay(String),

    #[fail(display = "Wrong date: '{}'.", _0)]
    WrongDate(String),
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
    use chrono::{Date, Utc};

    use crate::argv::{ParseArgv, ValidateArgv, YmdArgv};
    use chrono::offset::TimeZone;

    fn timestamp_of<Tz: TimeZone>(d: Date<Tz>) -> i64 {
        d.and_hms(0, 0, 0).timestamp()
    }

    #[test]
    fn parse() {
        let argv = YmdArgv::from(Utc);
        assert_eq!(
            argv.parse_argv("20190621").map(timestamp_of).unwrap_or(0),
            timestamp_of(Utc.ymd(2019, 6, 21))
        );
        assert_eq!(
            argv.parse_argv("2019-06-21").map(timestamp_of).unwrap_or(0),
            timestamp_of(Utc.ymd(2019, 6, 21))
        );
        assert_eq!(
            argv.parse_argv("2019/06/21").map(timestamp_of).unwrap_or(0),
            timestamp_of(Utc.ymd(2019, 6, 21))
        );
        assert_eq!(
            argv.parse_argv("2019/6/1").map(timestamp_of).unwrap_or(0),
            timestamp_of(Utc.ymd(2019, 6, 1))
        );

        assert!(argv.parse_argv("2020/2/29").is_ok());
        assert!(argv.parse_argv("2019/2/29").is_err());
    }

    #[test]
    fn validate() {
        assert!(YmdArgv::<Utc>::validate_argv("20190621".to_string()).is_ok());
        assert!(YmdArgv::<Utc>::validate_argv("2019-06-21".to_string()).is_ok());
        assert!(YmdArgv::<Utc>::validate_argv("2019/06/21".to_string()).is_ok());
        assert!(YmdArgv::<Utc>::validate_argv("2019/6/1".to_string()).is_ok());

        assert!(YmdArgv::<Utc>::validate_argv("2019".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("201906".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("2019061".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("201906011".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("18990101".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("30000101".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("29990001".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("29991301".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("29990132".to_string()).is_err());

        assert!(YmdArgv::<Utc>::validate_argv("2019".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("2019-".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("2019-06".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("2019-06-".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("2019-06-".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("-06-01".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("99999-06-01".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("2019--01".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("2019-123-01".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("2019-06-".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("2019-06-123".to_string()).is_err());
        assert!(YmdArgv::<Utc>::validate_argv("--".to_string()).is_err());
    }
}
