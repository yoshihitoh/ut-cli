use chrono::{Date, LocalResult, TimeZone};
use regex::Regex;

use crate::argv::{validate_number, ParseArgv, ValidateArgv};
use crate::error::{UtError, UtErrorKind};

fn extract_int(s: &str, start: usize, stop: usize) -> i32 {
    *&s[start..stop].parse().expect("not a number")
}

pub struct YmdArgv<Tz: TimeZone> {
    tz: Tz,
}

impl<Tz: TimeZone> From<Tz> for YmdArgv<Tz> {
    fn from(tz: Tz) -> Self {
        YmdArgv { tz }
    }
}

impl<Tz: TimeZone> ParseArgv<Date<Tz>> for YmdArgv<Tz> {
    fn parse_argv(&self, ymd: &str) -> Result<Date<Tz>, UtError> {
        let year_len = ymd.len() - 4;
        let y = extract_int(ymd, 0, year_len);
        let m = extract_int(ymd, year_len, year_len + 2);
        let d = extract_int(ymd, year_len + 2, year_len + 4);

        match self.tz.ymd_opt(y, m as u32, d as u32) {
            LocalResult::Single(date) => Ok(date),
            LocalResult::None => Err(UtError::from(UtErrorKind::WrongDate)),
            LocalResult::Ambiguous(_, _) => Err(UtError::from(UtErrorKind::AmbiguousDate)),
        }
    }
}

impl<Tz: TimeZone> ValidateArgv for YmdArgv<Tz> {
    fn validate_argv(ymd: String) -> Result<(), String> {
        let re = Regex::new(r"(\d{4})(\d{2})(\d{2})").expect("wrong regex pattern");
        let caps = re.captures(&ymd).ok_or(format!(
            "format must be \"yyyyMMdd\". given format: {}",
            ymd
        ))?;

        let y = caps.get(1).unwrap().as_str();
        validate_number("year", 1900, 2999, y)?;

        let m = caps.get(2).unwrap().as_str();
        validate_number("month", 1, 12, m)?;

        let d = caps.get(3).unwrap().as_str();
        validate_number("day", 1, 31, d)?;

        Ok(())
    }
}
