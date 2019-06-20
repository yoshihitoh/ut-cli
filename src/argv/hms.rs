use chrono::NaiveTime;
use regex::Regex;

use crate::argv::{validate_number, ParseArgv, ValidateArgv};
use crate::error::UtError;

pub struct HmsArgv {}

impl Default for HmsArgv {
    fn default() -> Self {
        HmsArgv {}
    }
}

impl ParseArgv<NaiveTime> for HmsArgv {
    fn parse_argv(&self, hms: &str) -> Result<NaiveTime, UtError> {
        let h = *&hms[0..2].parse::<u32>().expect("not a number");
        let m = *&hms[2..4].parse::<u32>().expect("not a number");
        let s = *&hms[4..6].parse::<u32>().expect("not a number");

        Ok(NaiveTime::from_hms(h, m, s))
    }
}

impl ValidateArgv for HmsArgv {
    fn validate_argv(hms: String) -> Result<(), String> {
        let re = Regex::new(r"(\d{2})(\d{2})(\d{2})").expect("wrong regex pattern");
        let caps = re
            .captures(&hms)
            .ok_or(format!("format must be \"HHmmss\". given format: {}", hms))?;

        let h = caps.get(1).unwrap().as_str();
        validate_number("hour", 0, 23, h)?;

        let m = caps.get(2).unwrap().as_str();
        validate_number("minute", 0, 59, m)?;

        let s = caps.get(3).unwrap().as_str();
        validate_number("second", 0, 59, s)?;

        Ok(())
    }
}
