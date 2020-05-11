use std::str::FromStr;

use regex::Regex;
use thiserror::Error;

use crate::find::FindByName;
use crate::timedelta::TimeDeltaBuilder;
use crate::unit::{TimeUnit, TimeUnitError};
use crate::validate::IntoValidationError;

#[derive(Error, Debug, PartialEq)]
pub enum DeltaItemError {
    #[error("Wrong format. error:{0}")]
    WrongFormat(String),

    #[error("Wrong value. error:{0}")]
    WrongValue(String),

    #[error("Wrong unit. error:{0}")]
    WrongUnit(TimeUnitError),
}

#[cfg(test)]
impl DeltaItemError {
    pub fn is_wrong_format(&self) -> bool {
        use DeltaItemError::*;
        match self {
            WrongFormat(_) => true,
            _ => false,
        }
    }

    pub fn is_wrong_value(&self) -> bool {
        use DeltaItemError::*;
        match self {
            WrongValue(_) => true,
            _ => false,
        }
    }

    pub fn is_wrong_unit(&self) -> bool {
        use DeltaItemError::*;
        match self {
            WrongUnit(_) => true,
            _ => false,
        }
    }
}

impl IntoValidationError for DeltaItemError {
    fn into_validation_error(self) -> String {
        use DeltaItemError::*;
        match &self {
            WrongFormat(_) => format!(
                "{} DELTA must consist of number and unit. See examples on help.",
                self
            ),
            WrongValue(_) => format!("{} DELTA value must be a number.", self),
            WrongUnit(e) => format!("{} {}", self, e),
        }
    }
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

    pub fn apply_timedelta_builder(self, builder: TimeDeltaBuilder) -> TimeDeltaBuilder {
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
                    .map_err(|e| DeltaItemError::WrongValue(e.to_string()));

                TimeUnit::find_by_name(caps.get(2).unwrap().as_str())
                    .map_err(DeltaItemError::WrongUnit)
                    .and_then(|unit| r_value.map(|value| DeltaItem { unit, value }))
            })
            .unwrap_or_else(|| Err(DeltaItemError::WrongFormat(s.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::delta::DeltaItem;
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

        let r = DeltaItem::from_str("+ 31d");
        assert!(r.is_err());
        assert!(r.err().unwrap().is_wrong_format());

        let r = DeltaItem::from_str("aa d");
        assert!(r.is_err());
        assert!(r.err().unwrap().is_wrong_format());

        let r = DeltaItem::from_str("12345678901d");
        assert!(r.is_err());
        assert!(r.err().unwrap().is_wrong_value());

        let r = DeltaItem::from_str("31b");
        assert!(r.is_err());
        assert!(r.err().unwrap().is_wrong_unit());
    }
}
