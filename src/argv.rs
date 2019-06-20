use crate::error::UtError;

mod delta;
mod hms;
mod precision;
mod preset;
mod unit;
mod ymd;

pub use delta::DeltaArgv;
pub use hms::HmsArgv;
pub use precision::PrecisionArgv;
pub use preset::PresetArgv;
pub use unit::TimeUnitArgv;
pub use ymd::YmdArgv;

pub trait ParseArgv<T> {
    fn parse_argv(&self, s: &str) -> Result<T, UtError>;
}

pub trait ValidateArgv {
    fn validate_argv(s: String) -> Result<(), String>;
}

fn validate_number(field_name: &str, min: i32, max: i32, text: &str) -> Result<(), String> {
    let number = text
        .parse::<i32>()
        .map_err(|_| format!("{} is not a number.", text))?;

    if number >= min && number <= max {
        Ok(())
    } else {
        Err(format!(
            "{} must be between {} and {} . given {}: {}",
            field_name, min, max, field_name, text
        ))
    }
}
