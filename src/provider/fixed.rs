use chrono::offset::TimeZone;
use chrono::{DateTime, FixedOffset, Utc};

use crate::provider::{DateTimeProvider, FromTimeZone};

pub struct FixedOffsetProvider {
    offset: FixedOffset,
}

impl DateTimeProvider<FixedOffset> for FixedOffsetProvider {
    fn timezone(&self) -> FixedOffset {
        self.offset
    }

    fn now(&self) -> DateTime<FixedOffset> {
        self.offset.from_utc_datetime(&Utc::now().naive_utc())
    }
}

impl FromTimeZone<FixedOffset> for FixedOffsetProvider {
    fn from_timezone(offset: FixedOffset) -> Self
    where
        Self: DateTimeProvider<FixedOffset>,
    {
        FixedOffsetProvider { offset }
    }
}
