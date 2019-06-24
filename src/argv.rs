use crate::error::UtError;

mod delta;
mod hms;
mod offset;
mod precision;
mod preset;
mod unit;
mod ymd;

pub use delta::DeltaArgv;
pub use hms::HmsArgv;
pub use offset::OffsetArgv;
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
