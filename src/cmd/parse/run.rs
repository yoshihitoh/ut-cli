use std::fmt::{Debug, Display};

use chrono::{Offset, TimeZone};
use clap::ArgMatches;

use crate::error::{UtError, UtErrorKind};
use crate::precision::Precision;
use crate::provider::DateTimeProvider;
use failure::ResultExt;

pub fn run<O, Tz, P>(m: &ArgMatches, provider: P, precision: Precision) -> Result<(), UtError>
where
    O: Offset + Display + Sized,
    Tz: TimeZone<Offset = O> + Debug,
    P: DateTimeProvider<Tz>,
{
    // TODO: create timestamp type.
    let timestamp = m
        .value_of("TIMESTAMP")
        .map(|s| s.parse::<i64>().map(Some))
        .unwrap_or_else(|| Ok(None))
        .context(UtErrorKind::WrongTimestamp)?
        .unwrap();

    let maybe_precision = m
        .value_of("PRECISION")
        .map(|s| Precision::find_by_name(s).map(Some))
        .unwrap_or_else(|| Ok(None))
        .context(UtErrorKind::PrecisionError)?;
    if maybe_precision.is_some() {
        eprintln!("-p PRECISION option is deprecated.");
    }
    let precision = maybe_precision.unwrap_or(precision);

    let dt = precision.parse_timestamp(provider.timezone(), timestamp);
    println!("{}", dt.format(precision.preferred_format()).to_string());
    Ok(())
}
