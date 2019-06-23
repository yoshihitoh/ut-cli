use failure::ResultExt;

use crate::argv::{ParseArgv, ValidateArgv};
use crate::error::{UtError, UtErrorKind};
use crate::find::FindError;
use crate::unit::TimeUnit;

pub struct TimeUnitArgv {}

impl Default for TimeUnitArgv {
    fn default() -> Self {
        TimeUnitArgv {}
    }
}

impl ParseArgv<TimeUnit> for TimeUnitArgv {
    fn parse_argv(&self, s: &str) -> Result<TimeUnit, UtError> {
        Ok(TimeUnit::find_by_name(s).context(UtErrorKind::PrecisionError)?)
    }
}

impl ValidateArgv for TimeUnitArgv {
    fn validate_argv(s: String) -> Result<(), String> {
        TimeUnit::find_by_name(&s).map(|_| ()).map_err(|e| match e {
            FindError::NotFound => {
                let names = TimeUnit::possible_names();
                format!("{} possible names: [{}]", e, names.join(", "))
            }
            FindError::Ambiguous(_) => format!("{}", e),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::argv::{ParseArgv, TimeUnitArgv, ValidateArgv};
    use crate::unit::TimeUnit;

    #[test]
    fn parse() {
        let argv = TimeUnitArgv::default();
        assert_eq!(argv.parse_argv("year").ok(), Some(TimeUnit::Year));
        assert!(argv.parse_argv("m").is_err());
        assert!(argv.parse_argv("week").is_err());
    }

    #[test]
    fn validate() {
        let validate_argv = |s: &str| TimeUnitArgv::validate_argv(s.to_string());

        assert!(validate_argv("year").is_ok());
        assert!(validate_argv("m").is_err());
        assert!(validate_argv("week").is_err());
    }
}
