use chrono::{DateTime, TimeZone};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use thiserror::Error;

use crate::find::{FindByName, FindError, PossibleNames, PossibleValues};
use crate::validate::IntoValidationError;

#[derive(Error, Debug, PartialEq)]
pub enum PrecisionError {
    #[error("Wrong precision. error:{0}")]
    WrongName(FindError),
}

impl From<FindError> for PrecisionError {
    fn from(e: FindError) -> Self {
        PrecisionError::WrongName(e)
    }
}

impl IntoValidationError for PrecisionError {
    fn into_validation_error(self) -> String {
        use PrecisionError::*;
        match &self {
            WrongName(e) => match e {
                FindError::NotFound => {
                    let names = Precision::possible_names();
                    format!("{} possible names: [{}]", self, names.join(", "))
                }
                _ => format!("{}", self),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, EnumIter, EnumString, Display)]
pub enum Precision {
    #[strum(serialize = "second", serialize = "s")]
    Second,

    #[strum(serialize = "millisecond", serialize = "ms")]
    MilliSecond,

    #[strum(serialize = "microsecond", serialize = "us")]
    MicroSecond,

    #[strum(serialize = "nanosecond", serialize = "ns")]
    NanoSecond,
}

impl Precision {
    pub fn parse_timestamp<Tz: TimeZone>(self, tz: Tz, timestamp: i64) -> DateTime<Tz> {
        match self {
            Precision::Second => tz.timestamp_opt(timestamp, 0).single(),
            Precision::MilliSecond => tz.timestamp_millis_opt(timestamp).single(),
            Precision::MicroSecond => tz.timestamp_micros(timestamp).single(),
            Precision::NanoSecond => Some(tz.timestamp_nanos(timestamp)),
        }
        .expect("invalid timestamp")
    }

    pub fn to_timestamp<Tz: TimeZone>(self, dt: DateTime<Tz>) -> i64 {
        match self {
            Precision::Second => dt.timestamp(),
            Precision::MilliSecond => dt.timestamp_millis(),
            Precision::MicroSecond => dt.timestamp_micros(),
            Precision::NanoSecond => dt.timestamp_nanos_opt().expect("invalid timestamp"),
        }
    }

    pub fn preferred_format(self) -> &'static str {
        match self {
            Precision::Second => "%Y-%m-%d %H:%M:%S (%Z)",
            Precision::MilliSecond => "%Y-%m-%d %H:%M:%S%.3f (%Z)",
            Precision::MicroSecond => "%Y-%m-%d %H:%M:%S%.6f (%Z)",
            Precision::NanoSecond => "%Y-%m-%d %H:%M:%S%.9f (%Z)",
        }
    }
}

impl PossibleNames for Precision {}

impl PossibleValues for Precision {
    type Iterator = PrecisionIter;

    fn possible_values() -> Self::Iterator {
        Precision::iter()
    }
}

impl FindByName for Precision {
    type Error = PrecisionError;
}

#[cfg(test)]
mod tests {
    use chrono::offset::TimeZone;
    use chrono::{Timelike, Utc};

    use crate::find::{FindByName, FindError};
    use crate::precision::{Precision, PrecisionError};

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
        assert_eq!(
            Precision::find_by_name("millis"),
            Ok(Precision::MilliSecond)
        );
        assert_eq!(Precision::find_by_name("ms"), Ok(Precision::MilliSecond));
    }

    #[test]
    fn find_by_name_microsecond() {
        assert_eq!(
            Precision::find_by_name("microsecond"),
            Ok(Precision::MicroSecond)
        );
        assert_eq!(
            Precision::find_by_name("micros"),
            Ok(Precision::MicroSecond)
        );
        assert_eq!(Precision::find_by_name("us"), Ok(Precision::MicroSecond));
    }

    #[test]
    fn find_by_name_nanosecond() {
        assert_eq!(
            Precision::find_by_name("nanosecond"),
            Ok(Precision::NanoSecond)
        );
        assert_eq!(Precision::find_by_name("n"), Ok(Precision::NanoSecond));
        assert_eq!(Precision::find_by_name("ns"), Ok(Precision::NanoSecond));
    }

    #[test]
    fn find_by_name_not_supported() {
        assert_eq!(
            Precision::find_by_name("year"),
            Err(PrecisionError::WrongName(FindError::NotFound))
        );
        assert_eq!(
            Precision::find_by_name("min"),
            Err(PrecisionError::WrongName(FindError::NotFound))
        );
    }

    #[test]
    fn parse_timestamp_second() {
        assert_eq!(
            Some(Precision::Second.parse_timestamp(Utc, 0)),
            Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).single()
        );

        assert_eq!(
            Some(Precision::Second.parse_timestamp(Utc, 1560762129123)),
            Utc.with_ymd_and_hms(51428, 8, 1, 11, 52, 3).single()
        );
    }

    #[test]
    fn parse_timestamp_millisecond() {
        assert_eq!(
            Some(Precision::MilliSecond.parse_timestamp(Utc, 0)),
            Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                .map(|dt| dt.with_nanosecond(0).unwrap())
                .single()
        );

        assert_eq!(
            Some(Precision::MilliSecond.parse_timestamp(Utc, 1560762129123)),
            Utc.with_ymd_and_hms(2019, 6, 17, 9, 2, 9)
                .map(|dt| dt.with_nanosecond(123_000_000).unwrap())
                .single()
        );
    }

    #[test]
    fn parse_timestamp_microsecond() {
        assert_eq!(
            Some(Precision::MicroSecond.parse_timestamp(Utc, 0)),
            Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                .map(|dt| dt.with_nanosecond(0).unwrap())
                .single()
        );

        assert_eq!(
            Some(Precision::MicroSecond.parse_timestamp(Utc, 1560762129123456)),
            Utc.with_ymd_and_hms(2019, 6, 17, 9, 2, 9)
                .map(|dt| dt.with_nanosecond(123_456_000).unwrap())
                .single()
        );
    }

    #[test]
    fn parse_timestamp_nanosecond() {
        assert_eq!(
            Some(Precision::NanoSecond.parse_timestamp(Utc, 0)),
            Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                .map(|dt| dt.with_nanosecond(0).unwrap())
                .single()
        );

        assert_eq!(
            Some(Precision::NanoSecond.parse_timestamp(Utc, 1560762129123456789)),
            Utc.with_ymd_and_hms(2019, 6, 17, 9, 2, 9)
                .map(|dt| dt.with_nanosecond(123_456_789).unwrap())
                .single()
        );
    }
}
