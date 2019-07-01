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
            // TODO: add validator
            Arg::with_name("PRECISION")
                .help("Set a precision of the timestamp.")
                .short("p")
                .long("precision")
                .takes_value(true)
                .default_value("second"),
        )
}

pub fn run<O, Tz, P>(m: &ArgMatches, provider: P) -> Result<(), UtError>
where
    O: Offset + Display + Sized,
    Tz: TimeZone<Offset = O>,
    P: DateTimeProvider<Tz>,
{
    let timestamp = parse_argv(TimestampArgv::default(), m.value_of("TIMESTAMP"))?.unwrap();
    let precision =
        parse_argv(PrecisionArgv::default(), m.value_of("PRECISION"))?.unwrap_or(Precision::Second);

    let dt = precision.parse_timestamp(provider.timezone(), timestamp);
    println!("{}", dt.format(precision.preferred_format()).to_string());
    Ok(())
}
