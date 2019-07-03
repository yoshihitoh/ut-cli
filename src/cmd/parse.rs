use chrono::{Offset, TimeZone};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::argv::{parse_argv, PrecisionArgv, TimestampArgv, ValidateArgv};
use crate::error::UtError;
use crate::precision::Precision;
use crate::provider::DateTimeProvider;
use std::fmt::Display;

pub fn command(name: &str) -> App<'static, 'static> {
    SubCommand::with_name(name)
        .about("Parse a unix timestamp and print it in human readable format.")
        .settings(&[AppSettings::AllowLeadingHyphen])
        .arg(
            Arg::with_name("TIMESTAMP")
                .help("Set a timestamp to parse.")
                .required(true)
                .validator(TimestampArgv::validate_argv)
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::with_name("PRECISION")
                .help("[Deprecated] Set a precision of the timestamp.")
                .short("p")
                .long("precision")
                .takes_value(true)
                .validator(PrecisionArgv::validate_argv),
        )
}

pub fn run<O, Tz, P>(m: &ArgMatches, provider: P) -> Result<(), UtError>
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
    let precision = maybe_precision.unwrap_or(Precision::Second);

    let dt = precision.parse_timestamp(provider.timezone(), timestamp);
    println!("{}", dt.format(precision.preferred_format()).to_string());
    Ok(())
}
