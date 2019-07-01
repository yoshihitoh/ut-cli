use crate::error::UtError;

mod delta;
mod hms;
mod offset;
mod precision;
mod preset;
mod timestamp;
mod unit;
mod ymd;

pub use self::delta::DeltaArgv;
pub use self::hms::HmsArgv;
pub use self::offset::OffsetArgv;
pub use self::precision::PrecisionArgv;
pub use self::preset::PresetArgv;
pub use self::timestamp::TimestampArgv;
pub use self::unit::TimeUnitArgv;
pub use self::ymd::YmdArgv;

pub trait ParseArgv<T> {
    fn parse_argv(&self, s: &str) -> Result<T, UtError>;
}

pub trait ValidateArgv {
    fn validate_argv(s: String) -> Result<(), String>;
}

pub fn parse_argv<P, T>(parser: P, maybe_text: Option<&str>) -> Result<Option<T>, UtError>
where
    P: ParseArgv<T>,
{
    maybe_text
        .map(|s| parser.parse_argv(s))
        .map_or(Ok(None), |r| r.map(Some))
}
