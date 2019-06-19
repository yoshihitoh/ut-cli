mod cmd;
mod delta;
mod error;
mod find;
mod precision;
mod preset;
mod unit;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings};

use error::UtError;

fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .settings(&[AppSettings::SubcommandRequiredElseHelp])
        .subcommand(cmd::generate::command("generate").alias("g"))
        .subcommand(cmd::parse::command("parse").alias("p"))
}

fn run() -> Result<(), UtError> {
    let app = app();
    let main_matches = app.get_matches();
    match main_matches.subcommand() {
        ("generate", generate_matches) => cmd::generate::run(generate_matches.unwrap()),
        ("parse", parse_matches) => cmd::parse::run(parse_matches.unwrap()),
        _ => panic!("never happen"),
    }
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("error: {}", e),
    }
}
