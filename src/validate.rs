use crate::find::{FindByName, FindError};
use std::str::FromStr;

pub fn validate_number<T: PartialOrd, E, F: Fn() -> E>(
    n: T,
    min: T,
    max: T,
    f: F,
) -> Result<(), E> {
    if n >= min && n <= max {
        Ok(())
    } else {
        Err(f())
    }
}

pub trait IntoValidationError {
    fn into_validation_error(self) -> String;
}

pub fn validate_argv<T, E>(s: String) -> Result<(), String>
where
    T: FromStr<Err = E>,
    E: IntoValidationError,
{
    T::from_str(s.as_ref())
        .map(|_| ())
        .map_err(|e| e.into_validation_error())
}

pub fn validate_argv_by_name<T, E>(s: String) -> Result<(), String>
where
    T: FindByName<Error = E>,
    E: From<FindError> + IntoValidationError + IntoValidationError,
{
    T::find_by_name(s.as_ref())
        .map(|_| ())
        .map_err(|e| e.into_validation_error())
}
