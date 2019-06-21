use failure::ResultExt;

use crate::argv::ParseArgv;
use crate::error::{UtError, UtErrorKind};
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
