use chrono::{Date, TimeZone};
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use crate::find::{enum_names, find_enum_item, FindError};
use crate::provider::DateTimeProvider;

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

    pub fn possible_names() -> Vec<String> {
        Preset::iter().map(|p| p.to_string()).collect()
    }

    pub fn as_date<P, Tz>(self, provider: &P) -> Date<Tz>
    where
        P: DateTimeProvider<Tz>,
        Tz: TimeZone,
    {
        match self {
            Preset::Today => provider.today(),
            Preset::Tomorrow => provider.tomorrow(),
            Preset::Yesterday => provider.yesterday(),
        }
    }
}
