use failure::{Backtrace, Context, Fail};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Fail)]
pub enum UtErrorKind {
    #[fail(display = "Time unit error.")]
    TimeUnitError,

    #[fail(display = "Preset error.")]
    PresetError,

    #[fail(display = "Delta error.")]
    DeltaError,

    #[fail(display = "Precision error.")]
    PrecisionError,

    #[fail(display = "Wrong date.")]
    WrongDate,

    #[fail(display = "Wrong time.")]
    WrongTime,

    #[fail(display = "Wrong time offset.")]
    WrongTimeOffset,

    #[fail(display = "Date is ambiguous.")]
    AmbiguousDate,
}

#[derive(Debug)]
pub struct UtError {
    inner: Context<UtErrorKind>,
}

impl Fail for UtError {
    fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for UtError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        Display::fmt(&self.inner, f)
    }
}

impl From<UtErrorKind> for UtError {
    fn from(kind: UtErrorKind) -> Self {
        UtError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<UtErrorKind>> for UtError {
    fn from(inner: Context<UtErrorKind>) -> Self {
        UtError { inner }
    }
}
