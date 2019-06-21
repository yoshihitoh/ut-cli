use failure::ResultExt;

use crate::argv::ParseArgv;
use crate::error::{UtError, UtErrorKind};
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
