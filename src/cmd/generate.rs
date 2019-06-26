use chrono::{Date, DateTime, NaiveTime, TimeZone, Utc};
use clap::{App, Arg, ArgMatches, SubCommand, Values};

use crate::argv::{
    DeltaArgv, HmsArgv, ParseArgv, PrecisionArgv, PresetArgv, TimeUnitArgv, ValidateArgv, YmdArgv,
};
use crate::delta::DeltaItem;
use crate::error::{UtError, UtErrorKind};
use crate::precision::Precision;
use crate::preset::{DateFixture, Preset};
use crate::timedelta::{ApplyDateTime, TimeDeltaBuilder};
use crate::unit::TimeUnit;

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
                .help("Set the precision of output timestamp.")
                .next_line_help(true)
                .short("p")
                .long("precision")
                .takes_value(true)
                .default_value("second")
                .validator(PrecisionArgv::validate_argv),
        )
}

pub fn run<Tz, F>(m: &ArgMatches, fixture: F) -> Result<(), UtError>
where
    Tz: TimeZone,
    F: DateFixture<Tz>,
{
    let maybe_preset = parse_argv(PresetArgv::default(), m.value_of("BASE"))?;
    let maybe_ymd = parse_argv(YmdArgv::from(fixture.timezone()), m.value_of("YMD"))?;
    let maybe_hms = parse_argv(HmsArgv::default(), m.value_of("HMS"))?;
    let maybe_truncate = parse_argv(TimeUnitArgv::default(), m.value_of("TRUNCATE"))?;

    let base = create_base_date(fixture, maybe_preset, maybe_ymd, maybe_hms, maybe_truncate)?;
    let deltas = create_deltas(m.values_of("DELTA"))?;
    let precision = parse_argv(PrecisionArgv::default(), m.value_of("PRECISION"))
        .map(|p| p.unwrap_or(Precision::Second))?;

    generate(Request {
        base,
        deltas,
        precision,
    })
}

struct Request<Tz: TimeZone> {
    base: DateTime<Tz>,
    deltas: Vec<DeltaItem>,
    precision: Precision,
}

fn parse_argv<P, T>(parser: P, maybe_text: Option<&str>) -> Result<Option<T>, UtError>
where
    P: ParseArgv<T>,
{
    maybe_text
        .map(|s| parser.parse_argv(s))
        .map_or(Ok(None), |r| r.map(Some))
}

fn create_base_date<F, Tz>(
    fixture: F,
    maybe_preset: Option<Preset>,
    maybe_ymd: Option<Date<Tz>>,
    maybe_hms: Option<NaiveTime>,
    maybe_truncate: Option<TimeUnit>,
) -> Result<DateTime<Tz>, UtError>
where
    Tz: TimeZone,
    F: DateFixture<Tz>,
{
    let now = fixture.now();

    let maybe_date = maybe_preset.map(|p| p.as_date(&fixture)).or(maybe_ymd);
    let has_date = maybe_date.is_some();
    let date = maybe_date.unwrap_or(now.date());
    let time = maybe_hms.unwrap_or(if has_date {
        NaiveTime::from_hms(0, 0, 0)
    } else {
        now.time()
    });

    Ok(maybe_truncate
        .iter()
        .fold(date.and_time(time).unwrap(), |dt, unit| unit.truncate(dt)))
}

fn create_deltas(maybe_values: Option<Values>) -> Result<Vec<DeltaItem>, UtError> {
    let delta_argv = DeltaArgv::default();
    maybe_values
        .map(|values| values.map(|s| delta_argv.parse_argv(s)).collect())
        .unwrap_or(Ok(Vec::new()))
}

fn generate<Tz: TimeZone>(request: Request<Tz>) -> Result<(), UtError> {
    let delta = request
        .deltas
        .into_iter()
        .fold(TimeDeltaBuilder::default(), |b, d| {
            d.apply_timedelta_builder(b)
        })
        .build();

    match delta.apply_datetime(request.base) {
        Some(dt) => {
            println!("{}", request.precision.to_timestamp(dt));
            Ok(())
        }
        None => Err(UtError::from(UtErrorKind::TimeUnitError)),
    }
}
