pub use std::fmt::Debug;

use chrono::{DateTime, NaiveTime, TimeZone};

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

    fn today(&self) -> DateTime<Tz> {
        self.now().with_time(NaiveTime::MIN).unwrap()
    }

    fn tomorrow(&self) -> DateTime<Tz> {
        add_days(self.today(), 1)
    }

    fn yesterday(&self) -> DateTime<Tz> {
        add_days(self.today(), -1)
    }
}

pub trait FromTimeZone<Tz: TimeZone + Debug> {
    fn from_timezone(tz: Tz) -> Self
    where
        Self: DateTimeProvider<Tz>;
}

fn add_days<Tz: TimeZone>(dt: DateTime<Tz>, days: i32) -> DateTime<Tz> {
    let delta = TimeDeltaBuilder::default().days(days).build();
    delta
        .apply_datetime(dt.clone())
        .unwrap_or_else(|| panic!("can't add days. dt={:?}, days={}", dt, days))
}
