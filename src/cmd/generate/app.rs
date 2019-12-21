use chrono::prelude::*;
use clap::{App, Arg, SubCommand};

use crate::argv::{
    DeltaArgv, HmsArgv, PrecisionArgv, PresetArgv, TimeUnitArgv, ValidateArgv, YmdArgv,
};

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
                .validator(PresetArgv::validate_argv)
                .conflicts_with("YMD"),
        )
        .arg(
            Arg::with_name("YMD")
                .value_name("DATE")
                .help("Set the DATE in yyyyMMdd format.")
                .long("ymd")
                .takes_value(true)
                .validator(YmdArgv::<Utc>::validate_argv)
                .conflicts_with("BASE"),
        )
        .arg(
            Arg::with_name("HMS")
                .value_name("TIME")
                .help("Set the TIME in HHmmss format.")
                .long("hms")
                .takes_value(true)
                .validator(HmsArgv::validate_argv),
        )
        .arg(
            Arg::with_name("TRUNCATE")
                .value_name("UNIT")
                .help("Set the UNIT to truncate the base DATE and TIME.")
                .next_line_help(true)
                .short("t")
                .long("truncate")
                .takes_value(true)
                .validator(TimeUnitArgv::validate_argv),
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
                .number_of_values(1)
                .validator(DeltaArgv::validate_argv),
        )
        .arg(
            Arg::with_name("PRECISION")
                .help("[Deprecated] Set the precision of output timestamp.")
                .next_line_help(true)
                .short("p")
                .long("precision")
                .takes_value(true)
                .validator(PrecisionArgv::validate_argv),
        )
}
