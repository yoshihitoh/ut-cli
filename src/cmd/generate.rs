use std::convert::TryFrom;

use chrono::{Local, TimeZone, Utc};
use clap::{App, Arg, ArgMatches, SubCommand};
use regex::Regex;

use timedelta::{ApplyDateTime, TimeDeltaBuilder};

use crate::error::{UtError, UtErrorKind};

mod request;
use request::Request;

fn validate_number(field_name: &str, min: i32, max: i32, text: &str) -> Result<(), String> {
    let number = text
        .parse::<i32>()
        .map_err(|_| format!("{} is not a number.", text))?;

    if number >= min && number <= max {
        Ok(())
    } else {
        Err(format!(
            "{} must be between {} and {} . given {}: {}",
            field_name, min, max, field_name, text
        ))
    }
}

fn validate_ymd(ymd: String) -> Result<(), String> {
    let re = Regex::new(r"(\d{4})(\d{2})(\d{2})").expect("wrong regex pattern");
    let caps = re.captures(&ymd).ok_or(format!(
        "format must be \"yyyyMMdd\". given format: {}",
        ymd
    ))?;

    let y = caps.get(1).unwrap().as_str();
    validate_number("year", 1900, 2999, y)?;

    let m = caps.get(2).unwrap().as_str();
    validate_number("month", 1, 12, m)?;

    let d = caps.get(3).unwrap().as_str();
    validate_number("day", 1, 31, d)?;

    Ok(())
}

fn validate_hms(hms: String) -> Result<(), String> {
    let re = Regex::new(r"(\d{2})(\d{2})(\d{2})").expect("wrong regex pattern");
    let caps = re
        .captures(&hms)
        .ok_or(format!("format must be \"HHmmss\". given format: {}", hms))?;

    let h = caps.get(1).unwrap().as_str();
    validate_number("hour", 0, 23, h)?;

    let m = caps.get(2).unwrap().as_str();
    validate_number("minute", 0, 59, m)?;

    let s = caps.get(3).unwrap().as_str();
    validate_number("second", 0, 59, s)?;

    Ok(())
}

pub fn command(name: &str) -> App<'static, 'static> {
    SubCommand::with_name(name)
        .about("Generate unix timestamp with given options.")
        .arg(
            Arg::with_name("BASE")
                .value_name("DATE")
                .help("Set base DATE from presets.")
                .next_line_help(true)
                .short("b")
                .long("base")
                .takes_value(true)
                .conflicts_with("YMD"),
        )
        .arg(
            Arg::with_name("YMD")
                .value_name("DATE")
                .help("Set the DATE in yyyyMMdd format.")
                .long("ymd")
                .takes_value(true)
                .validator(validate_ymd)
                .conflicts_with("BASE"),
        )
        .arg(
            Arg::with_name("HMS")
                .value_name("TIME")
                .help("Set the TIME in HHmmss format.")
                .long("hms")
                .takes_value(true)
                .validator(validate_hms),
        )
        .arg(
            Arg::with_name("TRUNCATE")
                .value_name("UNIT")
                .help("Set the UNIT to truncate the base DATE and TIME.")
                .next_line_help(true)
                .short("t")
                .long("truncate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("DELTA")
                .help("Set the timedelta consists of VALUE and UNIT.")
                .long_help(
                    "
Example:
    --delta=3day  :  3 days later.
    -d 1y -d -10h : 10 hours ago in next year.
",
                )
                .next_line_help(true)
                // TODO: long helpに使用例を追加
                .short("d")
                .long("delta")
                .takes_value(true)
                .allow_hyphen_values(true)
                .multiple(true)
                .number_of_values(1),
        )
        .arg(
            Arg::with_name("PRECISION")
                .help("Set the precision of output timestamp.")
                .next_line_help(true)
                .short("p")
                .long("precision")
                .takes_value(true)
                .default_value("second"),
        )
        .arg(
            Arg::with_name("UTC")
                .help("Use utc date and time on given options relate to date and time.")
                .short("u")
                .long("utc"),
        )
}

fn generate<Tz: TimeZone>(request: Request<Tz>) -> Result<(), UtError> {
    let delta = request
        .deltas()
        .into_iter()
        .fold(TimeDeltaBuilder::default(), |b, d| {
            d.apply_timedelta_builder(b)
        })
        .build();

    match delta.apply_datetime(request.base()) {
        Some(dt) => {
            println!("{}", request.precision().to_timestamp(dt));
            Ok(())
        }
        None => Err(UtError::from(UtErrorKind::TimeUnitError)),
    }
}

pub fn run(m: &ArgMatches<'static>) -> Result<(), UtError> {
    match m.value_of("UTC") {
        Some(_) => generate(Request::<Utc>::try_from(m)?),
        None => generate(Request::<Local>::try_from(m)?),
    }
}
