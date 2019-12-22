pub use std::fmt::Debug;

use chrono::{Date, DateTime, TimeZone};

use crate::timedelta::{ApplyDateTime, TimeDeltaBuilder};

mod fixed;
mod local;
mod utc;

pub use fixed::FixedOffsetProvider;
pub use local::LocalProvider;
pub use utc::UtcProvider;

pub trait DateTimeProvider<Tz: TimeZone + Debug> {
    fn timezone(&self) -> Tz;

    fn now(&self) -> DateTime<Tz>;

    fn today(&self) -> Date<Tz> {
        self.now().date()
    }

    fn tomorrow(&self) -> Date<Tz> {
        add_days(self.today(), 1)
    }

    fn yesterday(&self) -> Date<Tz> {
        add_days(self.today(), -1)
    }
}

pub trait FromTimeZone<Tz: TimeZone + Debug> {
    fn from_timezone(tz: Tz) -> Self
    where
        Self: DateTimeProvider<Tz>;
}

fn add_days<Tz: TimeZone>(date: Date<Tz>, days: i32) -> Date<Tz> {
    let delta = TimeDeltaBuilder::default().days(days).build();
    delta
        .apply_datetime(date.and_hms(0, 0, 0))
        .unwrap_or_else(|| panic!("can't add days. date={:?}, days={}", date, days))
        .date()
}
