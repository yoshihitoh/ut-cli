use chrono::{Date, DateTime, Local, TimeZone, Utc};
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use timedelta::{ApplyDateTime, TimeDeltaBuilder};

use crate::find::{enum_names, find_enum_item, FindError};

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, EnumString, Display)]
pub enum Preset {
    #[strum(serialize = "today")]
    Today,

    #[strum(serialize = "tomorrow")]
    Tomorrow,

    #[strum(serialize = "yesterday")]
    Yesterday,
}

lazy_static! {
    static ref PRESET_NAMES: Vec<String> = enum_names(Preset::iter());
    static ref POSSIBLE_VALUES: Vec<&'static str> =
        PRESET_NAMES.iter().map(|s| s.as_str()).collect();
}

impl Preset {
    pub fn find_by_name(name: &str) -> Result<Preset, FindError> {
        find_enum_item(&name.to_ascii_lowercase())
    }

    pub fn as_date<F, Tz>(&self, fixture: &F) -> Date<Tz>
    where
        F: DateFixture<Tz>,
        Tz: TimeZone,
    {
        match *self {
            Preset::Today => fixture.today(),
            Preset::Tomorrow => fixture.tomorrow(),
            Preset::Yesterday => fixture.yesterday(),
        }
    }
}

fn add_days<Tz: TimeZone>(date: Date<Tz>, days: i32) -> Date<Tz> {
    let delta = TimeDeltaBuilder::default().days(days).build();
    delta
        .apply_datetime(date.and_hms(0, 0, 0))
        .expect(&format!("can't add days. date={:?}, days={}", date, days))
        .date()
}

pub trait DateFixture<Tz: TimeZone> {
    fn timezone(&self) -> Tz;

    fn now(&self) -> DateTime<Tz>;

    fn today(&self) -> Date<Tz>;

    fn tomorrow(&self) -> Date<Tz> {
        add_days(self.today(), 1)
    }

    fn yesterday(&self) -> Date<Tz> {
        add_days(self.today(), -1)
    }
}

pub struct UtcDateFixture {}

impl Default for UtcDateFixture {
    fn default() -> Self {
        UtcDateFixture {}
    }
}

impl DateFixture<Utc> for UtcDateFixture {
    fn timezone(&self) -> Utc {
        Utc
    }

    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    fn today(&self) -> Date<Utc> {
        Utc::today()
    }
}

pub struct LocalDateFixture {}

impl Default for LocalDateFixture {
    fn default() -> Self {
        LocalDateFixture {}
    }
}

impl DateFixture<Local> for LocalDateFixture {
    fn timezone(&self) -> Local {
        Local
    }

    fn now(&self) -> DateTime<Local> {
        Local::now()
    }

    fn today(&self) -> Date<Local> {
        Local::today()
    }
}
