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

pub fn validate_argv<T, E>(s: &str) -> Result<(), String>
where
    T: FromStr<Err = E>,
    E: IntoValidationError,
{
    T::from_str(s)
        .map(|_| ())
        .map_err(|e| e.into_validation_error())
}
