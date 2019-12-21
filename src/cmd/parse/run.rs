use crate::argv::{parse_argv, PrecisionArgv, TimestampArgv};
use crate::error::UtError;
use crate::precision::Precision;
use crate::provider::DateTimeProvider;
use chrono::{Offset, TimeZone};
use clap::ArgMatches;
use std::fmt::Display;

pub fn run<O, Tz, P>(m: &ArgMatches, provider: P, precision: Precision) -> Result<(), UtError>
where
    O: Offset + Display + Sized,
    Tz: TimeZone<Offset = O>,
    P: DateTimeProvider<Tz>,
{
    let timestamp = parse_argv(TimestampArgv::default(), m.value_of("TIMESTAMP"))?.unwrap();

    let maybe_precision = parse_argv(PrecisionArgv::default(), m.value_of("PRECISION"))?;
    if maybe_precision.is_some() {
        eprintln!("-p PRECISION option is deprecated.");
    }
    let precision = maybe_precision.unwrap_or(precision);

    let dt = precision.parse_timestamp(provider.timezone(), timestamp);
    println!("{}", dt.format(precision.preferred_format()).to_string());
    Ok(())
}
