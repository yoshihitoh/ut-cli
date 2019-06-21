use chrono::Local;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use failure::Fail;
use regex::Regex;

use crate::error::{UtError, UtErrorKind};
use crate::precision::Precision;

pub fn command(name: &str) -> App<'static, 'static> {
    SubCommand::with_name(name)
        .about("Parse a unix timestamp and print it in human readable format.")
        .settings(&[AppSettings::AllowLeadingHyphen])
        .arg(
            Arg::with_name("TIMESTAMP")
                .required(true)
                .validator(is_timestamp)
                .allow_hyphen_values(true),
        )
        .arg(
            // TODO: add validator
            Arg::with_name("PRECISION")
                .short("p")
                .long_help("precision")
                .takes_value(true)
                .default_value("second"),
        )
}

pub fn run(m: &ArgMatches<'static>) -> Result<(), UtError> {
    let timestamp = m
        .value_of("TIMESTAMP")
        .unwrap()
        .parse::<i64>()
        .expect("not a number.");

    let precision = Precision::find_by_name(m.value_of("PRECISION").unwrap())
        .map_err(|e| e.context(UtErrorKind::PrecisionError))
        .map_err(UtError::from)?;

    let dt = precision.parse_timestamp(Local, timestamp);
    println!("{}", dt.format(precision.preferred_format()).to_string());
    Ok(())
}

fn is_timestamp(s: String) -> Result<(), String> {
    let re = Regex::new(r"[-+]?\d+").expect("wrong regex pattern.");
    if re.is_match(&s) {
        Ok(())
    } else {
        Err(format!("TIMESTAMP must be a number. given: {}", s))
    }
}
