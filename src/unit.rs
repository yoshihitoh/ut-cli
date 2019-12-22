use chrono::{DateTime, Datelike, TimeZone, Timelike};
use failure::Fail;
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use crate::find::{enum_names, find_enum_item, FindError};
use crate::validate::IntoValidationError;

lazy_static! {
    static ref PRESET_NAMES: Vec<String> = enum_names(TimeUnit::iter());
    static ref POSSIBLE_VALUES: Vec<&'static str> =
        PRESET_NAMES.iter().map(|s| s.as_str()).collect();
}

#[derive(Fail, Debug, PartialEq)]
pub enum TimeUnitError {
    #[fail(display = "Wrong unit. error:{}", _0)]
    WrongName(FindError),
}

impl IntoValidationError for TimeUnitError {
    fn into_validation_error(self) -> String {
        use TimeUnitError::*;
        match &self {
            WrongName(e) => match e {
                FindError::NotFound => {
                    let names = TimeUnit::possible_names();
                    format!("{} possible names: [{}]", self, names.join(", "))
                }
                FindError::Ambiguous(_) => format!("{}", self),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, EnumString, Display)]
pub enum TimeUnit {
    #[strum(serialize = "year")]
    Year,

    #[strum(serialize = "month")]
    Month,

    #[strum(serialize = "day")]
    Day,

    #[strum(serialize = "hour")]
    Hour,

    #[strum(serialize = "minute")]
    Minute,

    #[strum(serialize = "second")]
    Second,

    #[strum(serialize = "millisecond", serialize = "ms")]
    MilliSecond,
}

impl TimeUnit {
    pub fn find_by_name(name: &str) -> Result<TimeUnit, TimeUnitError> {
        find_enum_item(&name.to_ascii_lowercase()).map_err(TimeUnitError::WrongName)
    }

    pub fn possible_names() -> Vec<String> {
        TimeUnit::iter().map(|t| t.to_string()).collect()
    }

    pub fn truncate<Tz: TimeZone>(self, dt: DateTime<Tz>) -> DateTime<Tz> {
        let d = match self {
            TimeUnit::Year => dt.date().with_month(1).unwrap().with_day(1).unwrap(),
            TimeUnit::Month => dt.date().with_day(1).unwrap(),
            _ => dt.date(),
        };

        match self {
            TimeUnit::Hour => d.and_hms(dt.hour(), 0, 0),
            TimeUnit::Minute => d.and_hms(dt.hour(), dt.minute(), 0),
            TimeUnit::Second => d.and_hms(dt.hour(), dt.minute(), dt.second()),
            TimeUnit::MilliSecond => d.and_hms_milli(
                dt.hour(),
                dt.minute(),
                dt.second(),
                dt.timestamp_subsec_millis(),
            ),
            _ => d.and_hms(0, 0, 0),
        }
    }
}

#[cfg(test)]
mod find_tests {
    use crate::find::FindError;
    use crate::unit::{TimeUnit, TimeUnitError};

    #[test]
    fn find_by_name_year() {
        assert_eq!(TimeUnit::find_by_name("year"), Ok(TimeUnit::Year));
        assert_eq!(TimeUnit::find_by_name("YEAR"), Ok(TimeUnit::Year));
        assert_eq!(TimeUnit::find_by_name("y"), Ok(TimeUnit::Year));
    }

    #[test]
    fn find_by_name_month() {
        assert_eq!(TimeUnit::find_by_name("month"), Ok(TimeUnit::Month));
        assert_eq!(TimeUnit::find_by_name("mo"), Ok(TimeUnit::Month));

        assert_eq!(
            TimeUnit::find_by_name("m"),
            Err(TimeUnitError::WrongName(FindError::Ambiguous(vec![
                "month".to_string(),
                "minute".to_string(),
                "millisecond".to_string()
            ])))
        );
    }

    #[test]
    fn find_by_name_day() {
        assert_eq!(TimeUnit::find_by_name("day"), Ok(TimeUnit::Day));
        assert_eq!(TimeUnit::find_by_name("d"), Ok(TimeUnit::Day));
    }

    #[test]
    fn find_by_name_hour() {
        assert_eq!(TimeUnit::find_by_name("hour"), Ok(TimeUnit::Hour));
        assert_eq!(TimeUnit::find_by_name("h"), Ok(TimeUnit::Hour));
    }

    #[test]
    fn find_by_name_minute() {
        assert_eq!(TimeUnit::find_by_name("minute"), Ok(TimeUnit::Minute));
        assert_eq!(TimeUnit::find_by_name("min"), Ok(TimeUnit::Minute));

        assert_eq!(
            TimeUnit::find_by_name("mi"),
            Err(TimeUnitError::WrongName(FindError::Ambiguous(vec![
                "minute".to_string(),
                "millisecond".to_string()
            ])))
        );
    }

    #[test]
    fn find_by_name_second() {
        assert_eq!(TimeUnit::find_by_name("second"), Ok(TimeUnit::Second));
        assert_eq!(TimeUnit::find_by_name("s"), Ok(TimeUnit::Second));
    }

    #[test]
    fn find_by_name_milli_second() {
        assert_eq!(
            TimeUnit::find_by_name("millisecond"),
            Ok(TimeUnit::MilliSecond)
        );
        assert_eq!(TimeUnit::find_by_name("mil"), Ok(TimeUnit::MilliSecond));
        assert_eq!(TimeUnit::find_by_name("ms"), Ok(TimeUnit::MilliSecond));
    }

    #[test]
    fn find_by_name_not_supported() {
        assert_eq!(
            TimeUnit::find_by_name("b"),
            Err(TimeUnitError::WrongName(FindError::NotFound))
        );
    }
}

#[cfg(test)]
mod truncate_tests {
    use crate::unit::TimeUnit;

    use chrono::offset::TimeZone;
    use chrono::{DateTime, Utc};

    fn base_date() -> DateTime<Utc> {
        Utc.ymd(2019, 6, 17).and_hms_milli(11, 22, 33, 444)
    }

    #[test]
    fn truncate_year() {
        assert_eq!(
            TimeUnit::Year.truncate(base_date()),
            Utc.ymd(2019, 1, 1).and_hms(0, 0, 0)
        );

        assert_eq!(
            TimeUnit::Year.truncate(Utc.ymd(2019, 1, 1).and_hms(0, 0, 0)),
            Utc.ymd(2019, 1, 1).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn truncate_month() {
        assert_eq!(
            TimeUnit::Month.truncate(base_date()),
            Utc.ymd(2019, 6, 1).and_hms(0, 0, 0)
        );

        assert_eq!(
            TimeUnit::Month.truncate(Utc.ymd(2019, 6, 1).and_hms(0, 0, 0)),
            Utc.ymd(2019, 6, 1).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn truncate_day() {
        assert_eq!(
            TimeUnit::Day.truncate(base_date()),
            Utc.ymd(2019, 6, 17).and_hms(0, 0, 0)
        );

        assert_eq!(
            TimeUnit::Day.truncate(Utc.ymd(2019, 6, 17).and_hms(0, 0, 0)),
            Utc.ymd(2019, 6, 17).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn truncate_hour() {
        assert_eq!(
            TimeUnit::Hour.truncate(base_date()),
            Utc.ymd(2019, 6, 17).and_hms(11, 0, 0)
        );

        assert_eq!(
            TimeUnit::Hour.truncate(Utc.ymd(2019, 6, 17).and_hms(11, 0, 0)),
            Utc.ymd(2019, 6, 17).and_hms(11, 0, 0)
        );
    }

    #[test]
    fn truncate_minute() {
        assert_eq!(
            TimeUnit::Minute.truncate(base_date()),
            Utc.ymd(2019, 6, 17).and_hms(11, 22, 0)
        );

        assert_eq!(
            TimeUnit::Minute.truncate(Utc.ymd(2019, 6, 17).and_hms(11, 22, 0)),
            Utc.ymd(2019, 6, 17).and_hms(11, 22, 0)
        );
    }

    #[test]
    fn truncate_second() {
        assert_eq!(
            TimeUnit::Second.truncate(base_date()),
            Utc.ymd(2019, 6, 17).and_hms(11, 22, 33)
        );

        assert_eq!(
            TimeUnit::Second.truncate(Utc.ymd(2019, 6, 17).and_hms(11, 22, 33)),
            Utc.ymd(2019, 6, 17).and_hms(11, 22, 33)
        );
    }

    #[test]
    fn truncate_millisecond() {
        assert_eq!(
            TimeUnit::MilliSecond.truncate(base_date()),
            Utc.ymd(2019, 6, 17).and_hms_micro(11, 22, 33, 444_000)
        );

        assert_eq!(
            TimeUnit::MilliSecond.truncate(Utc.ymd(2019, 6, 17).and_hms_milli(11, 22, 33, 444)),
            Utc.ymd(2019, 6, 17).and_hms_milli(11, 22, 33, 444)
        );
    }
}
