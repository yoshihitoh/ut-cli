mod argv;
mod cmd;
mod config;
mod delta;
mod error;
mod find;
mod precision;
mod preset;
mod provider;
mod timedelta;
mod unit;

use std::fmt::Display;

use chrono::{Local, Offset, TimeZone, Utc};
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
};

use crate::argv::{OffsetArgv, ParseArgv, PrecisionArgv, ValidateArgv};
use crate::config::Config;
use crate::error::UtError;
use crate::precision::Precision;
use crate::provider::{
    DateTimeProvider, FixedOffsetProvider, FromTimeZone, LocalProvider, UtcProvider,
};

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
                .validator(OffsetArgv::validate_argv),
        )
        .arg(
            Arg::with_name("PRECISION")
                .help("Set the precision of output timestamp.")
                .next_line_help(true)
                .short("p")
                .long("precision")
                .takes_value(true)
                .validator(PrecisionArgv::validate_argv),
        )
}

fn config() -> Config {
    Config::from_env()
}

fn run() -> Result<(), UtError> {
    let app = app();
    let config = config();
    let main_matches = app.get_matches();
    let precision = PrecisionArgv::default().parse_argv(
        main_matches
            .value_of("PRECISION")
            .or_else(|| config.precision())
            .unwrap_or("second"),
    )?;

    if main_matches.is_present("UTC") {
        let provider: UtcProvider = UtcProvider::from_timezone(Utc);
        run_with(&main_matches, provider, precision)
    } else if let Some(offset_text) = main_matches.value_of("OFFSET").or_else(|| config.offset()) {
        let offset = OffsetArgv::default().parse_argv(offset_text)?;
        let provider: FixedOffsetProvider = FixedOffsetProvider::from_timezone(offset);
        run_with(&main_matches, provider, precision)
    } else {
        let provider: LocalProvider = LocalProvider::from_timezone(Local);
        run_with(&main_matches, provider, precision)
    }
}

fn run_with<O, Tz, P>(
    main_matches: &ArgMatches,
    provider: P,
    precision: Precision,
) -> Result<(), UtError>
where
    O: Offset + Display + Sized,
    Tz: TimeZone<Offset = O>,
    P: DateTimeProvider<Tz>,
{
    match main_matches.subcommand() {
        ("generate", generate_matches) => {
            cmd::generate::run(generate_matches.unwrap(), provider, precision)
        }
        ("parse", parse_matches) => cmd::parse::run(parse_matches.unwrap(), provider, precision),
        _ => panic!("never happen"),
    }
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("error: {}", e),
    }
}
