mod cmd;
mod config;
mod datetime;
mod delta;
mod find;
mod offset;
mod parse;
mod precision;
mod preset;
mod provider;
mod read;
mod timedelta;
mod unit;
mod validate;

use std::fmt::{Debug, Display};
use std::str::FromStr;

use anyhow::Context;
use chrono::{Local, TimeZone, Utc};
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
};

use crate::cmd::generate::GenerateRequest;
use crate::config::Config;
use crate::find::FindByName;
use crate::offset::{Offset, OffsetError};
use crate::precision::{Precision, PrecisionError};
use crate::provider::{
    DateTimeProvider, FixedOffsetProvider, FromTimeZone, LocalProvider, UtcProvider,
};
use crate::validate::{validate_argv, validate_argv_by_name};

fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .settings(&[
            AppSettings::SubcommandRequiredElseHelp,
            AppSettings::AllowNegativeNumbers,
            AppSettings::ColoredHelp,
        ])
        .subcommand(cmd::generate::command("generate").alias("g"))
        .subcommand(cmd::parse::command("parse").alias("p"))
        .arg(
            Arg::with_name("UTC")
                .help("Use utc timezone.")
                .short("u")
                .long("utc")
                .conflicts_with_all(&["OFFSET"]),
        )
        .arg(
            Arg::with_name("OFFSET")
                .help("Use given value as timezone offset.")
                .short("o")
                .long("offset")
                .takes_value(true)
                .allow_hyphen_values(true)
                .validator(validate_argv::<Offset, OffsetError>),
        )
        .arg(
            Arg::with_name("PRECISION")
                .help("Set the precision of output timestamp.")
                .next_line_help(true)
                .short("p")
                .long("precision")
                .takes_value(true)
                .validator(validate_argv_by_name::<Precision, PrecisionError>),
        )
}

fn config() -> Config {
    Config::from_env()
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = app();
    let config = config();
    let main_matches = app.get_matches();
    let maybe_precision = main_matches
        .value_of("PRECISION")
        .or_else(|| config.precision());
    let precision = Precision::find_by_name_opt(maybe_precision)
        .context("Precision error.")?
        .unwrap_or(Precision::Second);

    if main_matches.is_present("UTC") {
        let provider: UtcProvider = UtcProvider::from_timezone(Utc);
        run_with(&main_matches, provider, precision, &config)
    } else if let Some(offset_text) = main_matches.value_of("OFFSET").or_else(|| config.offset()) {
        let offset = Offset::from_str(offset_text)
            .context("Wrong time offset.")?
            .into();
        let provider: FixedOffsetProvider = FixedOffsetProvider::from_timezone(offset);
        run_with(&main_matches, provider, precision, &config)
    } else {
        let provider: LocalProvider = LocalProvider::from_timezone(Local);
        run_with(&main_matches, provider, precision, &config)
    }
}

fn run_with<O, Tz, P>(
    main_matches: &ArgMatches,
    provider: P,
    precision: Precision,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>>
where
    O: chrono::Offset + Display + Sized,
    Tz: TimeZone<Offset = O> + Debug,
    P: DateTimeProvider<Tz>,
{
    match main_matches.subcommand() {
        ("generate", generate_matches) => cmd::generate::run(GenerateRequest::new(
            generate_matches.unwrap(),
            provider,
            precision,
        )?),
        ("parse", parse_matches) => cmd::parse::run(cmd::parse::ParseRequest::new(
            parse_matches.unwrap(),
            provider,
            precision,
            config.datetime_format(),
        )?),
        _ => panic!("never happen"),
    }
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("error: {}", e),
    }
}
