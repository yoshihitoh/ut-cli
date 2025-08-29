use chrono::{DateTime, Datelike, TimeZone, Timelike};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use thiserror::Error;

use crate::find::{FindByName, FindError, PossibleNames, PossibleValues};
use crate::validate::IntoValidationError;

#[derive(Error, Debug, PartialEq)]
pub enum TimeUnitError {
    #[error("Wrong unit. error:{0}")]
    WrongName(FindError),
}

impl From<FindError> for TimeUnitError {
    fn from(e: FindError) -> Self {
        TimeUnitError::WrongName(e)
    }
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
    pub fn truncate<Tz: TimeZone>(self, dt: DateTime<Tz>) -> DateTime<Tz> {
        let d = match self {
            TimeUnit::Year => dt.date_naive().with_month(1).unwrap().with_day(1).unwrap(),
            TimeUnit::Month => dt.date_naive().with_day(1).unwrap(),
            _ => dt.date_naive(),
        };

        let naive_dt = match self {
            TimeUnit::Hour => d.and_hms_opt(dt.hour(), 0, 0),
            TimeUnit::Minute => d.and_hms_opt(dt.hour(), dt.minute(), 0),
            TimeUnit::Second => d.and_hms_opt(dt.hour(), dt.minute(), dt.second()),
            TimeUnit::MilliSecond => d.and_hms_milli_opt(
                dt.hour(),
                dt.minute(),
                dt.second(),
                dt.timestamp_subsec_millis(),
            ),
            _ => d.and_hms_opt(0, 0, 0),
        }
        .expect("wrong time");

        dt.timezone()
            .from_local_datetime(&naive_dt)
            .single()
            .expect("wrong timezone conversion")
    }
}

impl PossibleValues for TimeUnit {
    type Iterator = TimeUnitIter;

    fn possible_values() -> Self::Iterator {
        TimeUnit::iter()
    }
}

impl PossibleNames for TimeUnit {}

impl FindByName for TimeUnit {
    type Error = TimeUnitError;
}

#[cfg(test)]
mod find_tests {
    use crate::find::{FindByName, FindError};
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
    use chrono::{DateTime, Timelike, Utc};

    fn base_date() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2019, 6, 17, 11, 22, 33)
            .map(|dt| dt.with_nanosecond(444_000_000).expect("wrong nanosecond"))
            .single()
            .expect("wrong datetime")
    }

    fn utc_ymd_and_hms(y: i32, m: u32, d: u32, h: u32, min: u32, s: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, h, min, s)
            .single()
            .expect("wrong datetime")
    }

    fn utc_datetime_with_millis(dt: DateTime<Utc>, millis: u32) -> DateTime<Utc> {
        dt.with_nanosecond(millis * 1_000_000)
            .expect("wrong nanosecond")
    }

    #[test]
    fn truncate_year() {
        assert_eq!(
            TimeUnit::Year.truncate(base_date()),
            utc_ymd_and_hms(2019, 1, 1, 0, 0, 0)
        );

        assert_eq!(
            TimeUnit::Year.truncate(utc_ymd_and_hms(2019, 1, 1, 0, 0, 0)),
            utc_ymd_and_hms(2019, 1, 1, 0, 0, 0)
        );
    }

    #[test]
    fn truncate_month() {
        assert_eq!(
            TimeUnit::Month.truncate(base_date()),
            utc_ymd_and_hms(2019, 6, 1, 0, 0, 0)
        );

        assert_eq!(
            TimeUnit::Month.truncate(utc_ymd_and_hms(2019, 6, 1, 0, 0, 0)),
            utc_ymd_and_hms(2019, 6, 1, 0, 0, 0)
        );
    }

    #[test]
    fn truncate_day() {
        assert_eq!(
            TimeUnit::Day.truncate(base_date()),
            utc_ymd_and_hms(2019, 6, 17, 0, 0, 0)
        );

        assert_eq!(
            TimeUnit::Day.truncate(utc_ymd_and_hms(2019, 6, 17, 0, 0, 0)),
            utc_ymd_and_hms(2019, 6, 17, 0, 0, 0)
        );
    }

    #[test]
    fn truncate_hour() {
        assert_eq!(
            TimeUnit::Hour.truncate(base_date()),
            utc_ymd_and_hms(2019, 6, 17, 11, 0, 0)
        );

        assert_eq!(
            TimeUnit::Hour.truncate(utc_ymd_and_hms(2019, 6, 17, 11, 0, 0)),
            utc_ymd_and_hms(2019, 6, 17, 11, 0, 0)
        );
    }

    #[test]
    fn truncate_minute() {
        assert_eq!(
            TimeUnit::Minute.truncate(base_date()),
            utc_ymd_and_hms(2019, 6, 17, 11, 22, 0)
        );

        assert_eq!(
            TimeUnit::Minute.truncate(utc_ymd_and_hms(2019, 6, 17, 11, 22, 0)),
            utc_ymd_and_hms(2019, 6, 17, 11, 22, 0)
        );
    }

    #[test]
    fn truncate_second() {
        assert_eq!(
            TimeUnit::Second.truncate(base_date()),
            utc_ymd_and_hms(2019, 6, 17, 11, 22, 33)
        );

        assert_eq!(
            TimeUnit::Second.truncate(utc_ymd_and_hms(2019, 6, 17, 11, 22, 33)),
            utc_ymd_and_hms(2019, 6, 17, 11, 22, 33)
        );
    }

    #[test]
    fn truncate_millisecond() {
        assert_eq!(
            TimeUnit::MilliSecond.truncate(base_date()),
            utc_datetime_with_millis(utc_ymd_and_hms(2019, 6, 17, 11, 22, 33), 444)
        );

        assert_eq!(
            TimeUnit::MilliSecond.truncate(utc_datetime_with_millis(
                utc_ymd_and_hms(2019, 6, 17, 11, 22, 33),
                444
            )),
            utc_datetime_with_millis(utc_ymd_and_hms(2019, 6, 17, 11, 22, 33), 444)
        );
    }
}
