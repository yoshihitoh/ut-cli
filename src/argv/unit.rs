use failure::ResultExt;

use crate::argv::ParseArgv;
use crate::error::{UtError, UtErrorKind};
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
