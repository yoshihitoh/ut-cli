use crate::error::UtError;

mod delta;
mod hms;
mod offset;
mod precision;
mod preset;
mod unit;
mod ymd;

pub use self::delta::DeltaArgv;
pub use self::hms::HmsArgv;
pub use self::offset::OffsetArgv;
pub use self::precision::PrecisionArgv;
pub use self::preset::PresetArgv;
pub use self::unit::TimeUnitArgv;
pub use self::ymd::YmdArgv;

pub trait ParseArgv<T> {
    fn parse_argv(&self, s: &str) -> Result<T, UtError>;
}

pub trait ValidateArgv {
    fn validate_argv(s: String) -> Result<(), String>;
}
