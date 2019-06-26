use std::str::FromStr;

use failure::Fail;
use regex::Regex;

use crate::find::FindError;
use crate::timedelta::TimeDeltaBuilder;
use crate::unit::TimeUnit;

#[derive(Fail, Debug, PartialEq)]
pub enum DeltaItemError {
    #[fail(display = "Wrong delta format: {}", _0)]
    WrongFormat(String),

    #[fail(display = "Parse int error: {}", _0)]
    ParseInt(String),

    #[fail(display = "TimeUnit find error: {}", _0)]
    TimeUnitFindError(FindError),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DeltaItem {
    unit: TimeUnit,
    value: i32,
}

impl DeltaItem {
    #[cfg(test)]
    pub fn new(unit: TimeUnit, value: i32) -> DeltaItem {
        DeltaItem { unit, value }
    }

    pub fn apply_timedelta_builder(&self, builder: TimeDeltaBuilder) -> TimeDeltaBuilder {
        match self.unit {
            TimeUnit::Year => builder.add_years(self.value),
            TimeUnit::Month => builder.add_months(self.value),
            TimeUnit::Day => builder.add_days(self.value),
            TimeUnit::Hour => builder.add_hours(self.value),
            TimeUnit::Minute => builder.add_minutes(self.value),
            TimeUnit::Second => builder.add_seconds(self.value),
            TimeUnit::MilliSecond => builder.add_milliseconds(self.value),
        }
    }
}

impl FromStr for DeltaItem {
    type Err = DeltaItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^([-+]?\d+)([a-zA-Z]+)$").expect("wrong regex pattern.");
        let maybe_caps = re.captures(s);

        maybe_caps
            .map(|caps| {
                let r_value = caps
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .map_err(|e| DeltaItemError::ParseInt(e.to_string()));

                TimeUnit::find_by_name(caps.get(2).unwrap().as_str())
                    .map_err(DeltaItemError::TimeUnitFindError)
                    .and_then(|unit| r_value.map(|value| DeltaItem { unit, value }))
            })
            .unwrap_or(Err(DeltaItemError::WrongFormat(s.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::delta::{DeltaItem, DeltaItemError};
    use crate::find::FindError;
    use crate::unit::TimeUnit;

    #[test]
    fn delta_from_str() {
        assert_eq!(
            DeltaItem::from_str("12y"),
            Ok(DeltaItem::new(TimeUnit::Year, 12))
        );
        assert_eq!(
            DeltaItem::from_str("-10mon"),
            Ok(DeltaItem::new(TimeUnit::Month, -10))
        );
        assert_eq!(
            DeltaItem::from_str("+31d"),
            Ok(DeltaItem::new(TimeUnit::Day, 31))
        );

        assert_eq!(
            DeltaItem::from_str("+ 31d"),
            Err(DeltaItemError::WrongFormat("+ 31d".to_string()))
        );

        assert_eq!(
            DeltaItem::from_str("aa d"),
            Err(DeltaItemError::WrongFormat("aa d".to_string()))
        );

        assert!(DeltaItem::from_str("12345678901d")
            .err()
            .map(|e| match e {
                DeltaItemError::ParseInt(_) => true,
                _ => false,
            })
            .unwrap_or(false));

        assert_eq!(
            DeltaItem::from_str("31b"),
            Err(DeltaItemError::TimeUnitFindError(FindError::NotFound))
        );
    }
}
