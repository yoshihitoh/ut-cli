use clap::{App, AppSettings, Arg, SubCommand};

use crate::datetime::{Hms, HmsError, Ymd, YmdError};
use crate::delta::{DeltaItem, DeltaItemError};
use crate::precision::{Precision, PrecisionError};
use crate::preset::{Preset, PresetError};
use crate::unit::{TimeUnit, TimeUnitError};
use crate::validate::{validate_argv, validate_argv_by_name};

pub fn command(name: &str) -> App<'static, 'static> {
    SubCommand::with_name(name)
        .about("Generate unix timestamp with given options.")
        .settings(&[AppSettings::AllowNegativeNumbers, AppSettings::ColoredHelp])
        .arg(
            Arg::with_name("BASE")
                .value_name("DATE")
                .help("Set base DATE from presets.")
                .next_line_help(true)
                .short("b")
                .long("base")
                .takes_value(true)
                .validator(validate_argv_by_name::<Preset, PresetError>)
                .conflicts_with_all(&["BASE_TIMESTAMP", "YMD"]),
        )
        .arg(
            Arg::with_name("BASE_TIMESTAMP")
                .help("Set a base timestamp.")
                .validator(|s| s.parse::<i64>().map(|_| ()).map_err(|e| format!("{:?}", e)))
                .allow_hyphen_values(true)
                .conflicts_with_all(&["BASE", "YMD", "HMS"]),
        )
        .arg(
            Arg::with_name("YMD")
                .value_name("DATE")
                .help("Set the DATE in yyyyMMdd format.")
                .long("ymd")
                .takes_value(true)
                .validator(validate_argv::<Ymd, YmdError>),
        )
        .arg(
            Arg::with_name("HMS")
                .value_name("TIME")
                .help("Set the TIME in HHmmss format.")
                .long("hms")
                .takes_value(true)
                .validator(validate_argv::<Hms, HmsError>),
        )
        .arg(
            Arg::with_name("TRUNCATE")
                .value_name("UNIT")
                .help("Set the UNIT to truncate the base DATE and TIME.")
                .next_line_help(true)
                .short("t")
                .long("truncate")
                .takes_value(true)
                .validator(validate_argv_by_name::<TimeUnit, TimeUnitError>),
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
                .validator(validate_argv::<DeltaItem, DeltaItemError>),
        )
        .arg(
            Arg::with_name("PRECISION")
                .help("[Deprecated] Set the precision of output timestamp.")
                .next_line_help(true)
                .short("p")
                .long("precision")
                .takes_value(true)
                .validator(validate_argv_by_name::<Precision, PrecisionError>),
        )
}
