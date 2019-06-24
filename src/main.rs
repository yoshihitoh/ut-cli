mod argv;
mod cmd;
mod delta;
mod error;
mod find;
mod precision;
mod preset;
mod unit;

use std::fmt::Display;

use chrono::{Offset, TimeZone};
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
};

use argv::{ParseArgv, ValidateArgv};
use error::UtError;
use preset::{DateFixture, LocalDateFixture, UtcDateFixture};

fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .settings(&[
            AppSettings::SubcommandRequiredElseHelp,
            AppSettings::AllowNegativeNumbers,
        ])
        .subcommand(cmd::generate::command("generate").alias("g"))
        .subcommand(cmd::parse::command("parse").alias("p"))
        .arg(
            Arg::with_name("UTC")
                .help("Use utc timezone.")
                .short("u")
                .long("utc"),
        )
}

fn run() -> Result<(), UtError> {
    let app = app();
    let main_matches = app.get_matches();

    if main_matches.is_present("UTC") {
        run_with(&main_matches, UtcDateFixture::default())
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
