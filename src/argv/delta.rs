use std::str::FromStr;

use failure::ResultExt;

use crate::argv::ParseArgv;
use crate::delta::DeltaItem;
use crate::error::{UtError, UtErrorKind};

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
