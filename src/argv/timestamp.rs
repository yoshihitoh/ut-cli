use failure::ResultExt;

use crate::argv::{ParseArgv, ValidateArgv};
use crate::error::{UtError, UtErrorKind};

pub struct TimestampArgv {}

impl Default for TimestampArgv {
    fn default() -> Self {
        TimestampArgv {}
    }
}

impl ParseArgv<i64> for TimestampArgv {
    fn parse_argv(&self, s: &str) -> Result<i64, UtError> {
        Ok(s.parse::<i64>().context(UtErrorKind::WrongTimestamp)?)
    }
}

impl ValidateArgv for TimestampArgv {
    fn validate_argv(s: String) -> Result<(), String> {
        s.parse::<i32>()
            .map(|_| ())
            .map_err(|_| format!("TIMESTAMP must be a number. given: {}", s))
    }
}
