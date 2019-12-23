use std::fmt::Debug;

use chrono::{Date, TimeZone};
use failure::Fail;
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use crate::find::{enum_names, FindByName, FindError, PossibleValues};
use crate::provider::DateTimeProvider;
use crate::validate::IntoValidationError;

#[derive(Fail, Debug, PartialEq)]
pub enum PresetError {
    #[fail(display = "Wrong preset. error:{}", _0)]
    WrongName(FindError),
}

impl From<FindError> for PresetError {
    fn from(e: FindError) -> Self {
        PresetError::WrongName(e)
    }
}

impl IntoValidationError for PresetError {
    fn into_validation_error(self) -> String {
        use PresetError::*;
        match &self {
            WrongName(e) => match e {
                FindError::NotFound => {
                    let names = Preset::possible_names();
                    format!("{} possible names: [{}]", self, names.join(", "))
                }
                _ => format!("{}", self),
            },
        }
    }
}

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
    pub fn possible_names() -> Vec<String> {
        Preset::iter().map(|p| p.to_string()).collect()
    }

    pub fn as_date<P, Tz>(self, provider: &P) -> Date<Tz>
    where
        P: DateTimeProvider<Tz>,
        Tz: TimeZone + Debug,
    {
        match self {
            Preset::Today => provider.today(),
            Preset::Tomorrow => provider.tomorrow(),
            Preset::Yesterday => provider.yesterday(),
        }
    }
}

impl PossibleValues for Preset {
    type Iterator = PresetIter;

    fn possible_values() -> Self::Iterator {
        Preset::iter()
    }
}

impl FindByName for Preset {
    type Error = PresetError;
}
