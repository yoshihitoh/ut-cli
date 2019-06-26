mod argv;
mod cmd;
mod delta;
mod error;
mod find;
mod precision;
mod preset;
mod timedelta;
mod unit;

use std::fmt::Display;

use chrono::{Offset, TimeZone};
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
};

use crate::argv::{OffsetArgv, ParseArgv, ValidateArgv};
use crate::error::UtError;
use crate::preset::FixedOffsetDateFixture;
use crate::preset::{DateFixture, LocalDateFixture, UtcDateFixture};

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
}

fn run() -> Result<(), UtError> {
    let app = app();
    let main_matches = app.get_matches();

    if main_matches.is_present("UTC") {
        run_with(&main_matches, UtcDateFixture::default())
    } else if let Some(offset_name) = main_matches.value_of("OFFSET") {
        let offset = OffsetArgv::default().parse_argv(offset_name)?;
        run_with(&main_matches, FixedOffsetDateFixture::from(offset))
    } else {
        run_with(&main_matches, LocalDateFixture::default())
    }
}

fn run_with<O, Tz, F>(main_matches: &ArgMatches, fixture: F) -> Result<(), UtError>
where
    O: Offset + Display + Sized,
    Tz: TimeZone<Offset = O>,
    F: DateFixture<Tz>,
{
    match main_matches.subcommand() {
        ("generate", generate_matches) => cmd::generate::run(generate_matches.unwrap(), fixture),
        ("parse", parse_matches) => cmd::parse::run(parse_matches.unwrap(), fixture),
        _ => panic!("never happen"),
    }
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("error: {}", e),
    }
}
