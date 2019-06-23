use failure::ResultExt;

use crate::argv::{ParseArgv, ValidateArgv};
use crate::error::{UtError, UtErrorKind};
use crate::find::FindError;
use crate::preset::Preset;

pub struct PresetArgv {}

impl Default for PresetArgv {
    fn default() -> Self {
        PresetArgv {}
    }
}

impl ParseArgv<Preset> for PresetArgv {
    fn parse_argv(&self, s: &str) -> Result<Preset, UtError> {
        Ok(Preset::find_by_name(s).context(UtErrorKind::PresetError)?)
    }
}

impl ValidateArgv for PresetArgv {
    fn validate_argv(s: String) -> Result<(), String> {
        Preset::find_by_name(&s).map(|_| ()).map_err(|e| match e {
            FindError::NotFound => {
                let names = Preset::possible_names();
                format!("{} possible names: [{}]", e, names.join(", "))
            }
            FindError::Ambiguous(_) => format!("{}", e),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::argv::{ParseArgv, PresetArgv, ValidateArgv};
    use crate::preset::Preset;

    #[test]
    fn parse() {
        let argv = PresetArgv::default();

        assert_eq!(argv.parse_argv("today").ok(), Some(Preset::Today));

        assert!(argv.parse_argv("t").is_err());
        assert!(argv.parse_argv("a").is_err());
    }

    #[test]
    fn validate() {
        let validate_argv = |s: &str| PresetArgv::validate_argv(s.to_string());

        assert!(validate_argv("today").is_ok());

        assert!(validate_argv("t").is_err());
        assert!(validate_argv("a").is_err());
    }
}
