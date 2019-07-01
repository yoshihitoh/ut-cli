use chrono::{DateTime, Utc};

use crate::provider::{DateTimeProvider, FromTimeZone};

pub struct UtcProvider {}

impl DateTimeProvider<Utc> for UtcProvider {
    fn timezone(&self) -> Utc {
        Utc
    }

    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

impl FromTimeZone<Utc> for UtcProvider {
    fn from_timezone(_tz: Utc) -> Self
    where
        Self: DateTimeProvider<Utc>,
    {
        UtcProvider {}
    }
}
