use failure::ResultExt;

use crate::argv::{ParseArgv, ValidateArgv};
use crate::error::{UtError, UtErrorKind};
use crate::find::FindError;
use crate::precision::Precision;

pub struct PrecisionArgv {}

impl Default for PrecisionArgv {
    fn default() -> Self {
        PrecisionArgv {}
    }
}

impl ParseArgv<Precision> for PrecisionArgv {
    fn parse_argv(&self, s: &str) -> Result<Precision, UtError> {
        Ok(Precision::find_by_name(s).context(UtErrorKind::PrecisionError)?)
    }
}

impl ValidateArgv for PrecisionArgv {
    fn validate_argv(s: String) -> Result<(), String> {
        Precision::find_by_name(&s)
            .map(|_| ())
            .map_err(|e| match e {
                FindError::NotFound => {
                    let names = Precision::possible_names();
                    format!("{} possible names: [{}]", e, names.join(", "))
                }
                FindError::Ambiguous(candidates) => format!(
                    "PRECISION is ambiguous. candidates: [{}:",
                    candidates.join(", ")
                ),
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::argv::{ParseArgv, PrecisionArgv, ValidateArgv};
    use crate::precision::Precision;

    #[test]
    fn parse() {
        let argv = PrecisionArgv::default();
        assert_eq!(argv.parse_argv("second").ok(), Some(Precision::Second));
    }

    #[test]
    fn validate() {
        let validate_argv = |s: &str| PrecisionArgv::validate_argv(s.to_string());
        assert!(validate_argv("second").is_ok());

        assert!(validate_argv("y").is_err());
        assert!(validate_argv("a").is_err());
    }
}
