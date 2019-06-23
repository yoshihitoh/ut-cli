use std::str::FromStr;

use failure::ResultExt;

use crate::argv::{ParseArgv, ValidateArgv};
use crate::delta::{DeltaItem, DeltaItemError};
use crate::error::{UtError, UtErrorKind};
use crate::find::FindError;
use crate::unit::TimeUnit;

pub struct DeltaArgv {}

impl Default for DeltaArgv {
    fn default() -> Self {
        DeltaArgv {}
    }
}

impl ParseArgv<DeltaItem> for DeltaArgv {
    fn parse_argv(&self, s: &str) -> Result<DeltaItem, UtError> {
        Ok(DeltaItem::from_str(s).context(UtErrorKind::DeltaError)?)
    }
}

impl ValidateArgv for DeltaArgv {
    fn validate_argv(s: String) -> Result<(), String> {
        DeltaItem::from_str(&s).map(|_| ()).map_err(|e| match e {
            DeltaItemError::WrongFormat(_) => format!(
                "{} <DELTA> must consist of NUMBER and UNIT. See examples on help.",
                e
            ),
            DeltaItemError::ParseInt(_) => format!("{}", e),
            DeltaItemError::TimeUnitFindError(e) => match e {
                FindError::NotFound => {
                    let names = TimeUnit::possible_names();
                    format!(
                        "No matching UNIT found. possible units: [{}]",
                        names.join(", ")
                    )
                }
                FindError::Ambiguous(candidates) => {
                    format!("UNIT is ambiguous. candidates: [{}]", candidates.join(", "))
                }
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::argv::{DeltaArgv, ParseArgv, ValidateArgv};
    use crate::delta::DeltaItem;
    use crate::unit::TimeUnit;

    #[test]
    fn parse() {
        let argv = DeltaArgv::default();
        assert_eq!(
            argv.parse_argv("1y").ok(),
            Some(DeltaItem::new(TimeUnit::Year, 1))
        );
        assert_eq!(
            argv.parse_argv("+2mon").ok(),
            Some(DeltaItem::new(TimeUnit::Month, 2))
        );
        assert_eq!(
            argv.parse_argv("-3d").ok(),
            Some(DeltaItem::new(TimeUnit::Day, -3))
        );
    }

    #[test]
    fn validate() {
        let validate_argv = |s: &str| DeltaArgv::validate_argv(s.to_string());

        assert!(validate_argv("1year").is_ok());
        assert!(validate_argv("+2month").is_ok());
        assert!(validate_argv("-3day").is_ok());

        assert!(validate_argv("y").is_err());
        assert!(validate_argv("1").is_err());
        assert!(validate_argv("12346789012y").is_err());
        assert!(validate_argv("1a").is_err());
        assert!(validate_argv("*1y").is_err());
    }
}
