use chrono::{DateTime, Local};

use crate::provider::{DateTimeProvider, FromTimeZone};

pub struct LocalProvider {}

impl DateTimeProvider<Local> for LocalProvider {
    fn timezone(&self) -> Local {
        Local
    }

    fn now(&self) -> DateTime<Local> {
        Local::now()
    }
}

impl FromTimeZone<Local> for LocalProvider {
    fn from_timezone(_tz: Local) -> Self
    where
        Self: DateTimeProvider<Local>,
    {
        LocalProvider {}
    }
}
