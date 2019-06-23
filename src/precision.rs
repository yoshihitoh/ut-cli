use chrono::{DateTime, TimeZone};
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use crate::find::{enum_names, find_enum_item, FindError};

lazy_static! {
    static ref PRESET_NAMES: Vec<String> = enum_names(Precision::iter());
    static ref POSSIBLE_VALUES: Vec<&'static str> =
        PRESET_NAMES.iter().map(|s| s.as_str()).collect();
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, EnumString, Display)]
pub enum Precision {
    #[strum(serialize = "second")]
    Second,

    #[strum(serialize = "millisecond", serialize = "ms")]
    MilliSecond,
}

impl Precision {
    pub fn find_by_name(name: &str) -> Result<Precision, FindError> {
        find_enum_item(&name.to_ascii_lowercase())
    }

    pub fn possible_names() -> Vec<String> {
        Precision::iter().map(|p| p.to_string()).collect()
    }

    pub fn parse_timestamp<Tz: TimeZone>(&self, tz: Tz, timestamp: i64) -> DateTime<Tz> {
        match *self {
            Precision::Second => tz.timestamp(timestamp, 0),
            Precision::MilliSecond => tz.timestamp_millis(timestamp),
        }
    }

    pub fn to_timestamp<Tz: TimeZone>(&self, dt: DateTime<Tz>) -> i64 {
        match *self {
            Precision::Second => dt.timestamp(),
            Precision::MilliSecond => dt.timestamp_millis(),
        }
    }

    pub fn preferred_format(&self) -> &'static str {
        match *self {
            Precision::Second => "%Y-%m-%d %H:%M:%S (%Z)",
            Precision::MilliSecond => "%Y-%m-%d %H:%M:%S%.3f (%Z)",
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::offset::TimeZone;
    use chrono::Utc;

    use crate::find::FindError;
    use crate::precision::Precision;

    #[test]
    fn find_by_name_second() {
        assert_eq!(Precision::find_by_name("second"), Ok(Precision::Second));
        assert_eq!(Precision::find_by_name("s"), Ok(Precision::Second));
    }

    #[test]
    fn find_by_name_millisecond() {
        assert_eq!(
            Precision::find_by_name("millisecond"),
            Ok(Precision::MilliSecond)
        );
        assert_eq!(Precision::find_by_name("m"), Ok(Precision::MilliSecond));
        assert_eq!(Precision::find_by_name("ms"), Ok(Precision::MilliSecond));
    }

    #[test]
    fn find_by_name_not_supported() {
        assert_eq!(Precision::find_by_name("year"), Err(FindError::NotFound));
        assert_eq!(Precision::find_by_name("min"), Err(FindError::NotFound));
    }

    #[test]
    fn parse_timestamp_second() {
        assert_eq!(
            Precision::Second.parse_timestamp(Utc, 0),
            Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)
        );

        assert_eq!(
            Precision::Second.parse_timestamp(Utc, 1560762129123),
            Utc.ymd(51428, 8, 1).and_hms(11, 52, 3)
        );
    }

    #[test]
    fn parse_timestamp_millisecond() {
        assert_eq!(
            Precision::MilliSecond.parse_timestamp(Utc, 0),
            Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 0, 0)
        );

        assert_eq!(
            Precision::MilliSecond.parse_timestamp(Utc, 1560762129123),
            Utc.ymd(2019, 6, 17).and_hms_milli(9, 2, 9, 123)
        );
    }
}
