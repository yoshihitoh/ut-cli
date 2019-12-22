use crate::precision::Precision;
use crate::validate::IntoValidationError;
use clap::{App, AppSettings, Arg, SubCommand};

pub fn command(name: &str) -> App<'static, 'static> {
    SubCommand::with_name(name)
        .about("Parse a unix timestamp and print it in human readable format.")
        .settings(&[AppSettings::AllowLeadingHyphen])
        .arg(
            Arg::with_name("TIMESTAMP")
                .help("Set a timestamp to parse.")
                .required(true)
                .validator(|s| s.parse::<i64>().map(|_| ()).map_err(|e| format!("{:?}", e)))
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::with_name("PRECISION")
                .help("[Deprecated] Set a precision of the timestamp.")
                .short("p")
                .long("precision")
                .takes_value(true)
                .validator(|s| {
                    Precision::find_by_name(&s)
                        .map(|_| ())
                        .map_err(|e| e.into_validation_error())
                }),
        )
}
