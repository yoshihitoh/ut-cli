use clap::{App, Arg, SubCommand};

use crate::datetime::{Hms, HmsError, Ymd, YmdError};
use crate::delta::{DeltaItem, DeltaItemError};
use crate::precision::Precision;
use crate::preset::Preset;
use crate::unit::TimeUnit;
use crate::validate::{validate_argv, IntoValidationError};

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
                .validator(|s| {
                    Preset::find_by_name(&s)
                        .map(|_| ())
                        .map_err(|e| e.into_validation_error())
                })
                .conflicts_with("YMD"),
        )
        .arg(
            Arg::with_name("YMD")
                .value_name("DATE")
                .help("Set the DATE in yyyyMMdd format.")
                .long("ymd")
                .takes_value(true)
                .validator(|s| validate_argv::<Ymd, YmdError>(&s))
                .conflicts_with("BASE"),
        )
        .arg(
            Arg::with_name("HMS")
                .value_name("TIME")
                .help("Set the TIME in HHmmss format.")
                .long("hms")
                .takes_value(true)
                .validator(|s| validate_argv::<Hms, HmsError>(&s)),
        )
        .arg(
            Arg::with_name("TRUNCATE")
                .value_name("UNIT")
                .help("Set the UNIT to truncate the base DATE and TIME.")
                .next_line_help(true)
                .short("t")
                .long("truncate")
                .takes_value(true)
                .validator(|s| {
                    TimeUnit::find_by_name(&s)
                        .map(|_| ())
                        .map_err(|e| e.into_validation_error())
                }),
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
                .short("d")
                .long("delta")
                .takes_value(true)
                .allow_hyphen_values(true)
                .multiple(true)
                .number_of_values(1)
                .validator(|s| validate_argv::<DeltaItem, DeltaItemError>(&s)),
        )
        .arg(
            Arg::with_name("PRECISION")
                .help("[Deprecated] Set the precision of output timestamp.")
                .next_line_help(true)
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
