use std::fmt::Debug;
use std::str::FromStr;

use chrono::prelude::*;
use clap::{ArgMatches, Values};
use failure::ResultExt;

use crate::datetime::{Hms, HmsError, Ymd, YmdError};
use crate::delta::DeltaItem;
use crate::error::{UtError, UtErrorKind};
use crate::find::FindByName;
use crate::parse::parse_argv_opt;
use crate::precision::Precision;
use crate::preset::Preset;
use crate::provider::DateTimeProvider;
use crate::timedelta::{ApplyDateTime, TimeDeltaBuilder};
use crate::unit::TimeUnit;

pub fn run<Tz, P>(m: &ArgMatches, provider: P, precision: Precision) -> Result<(), UtError>
where
    Tz: TimeZone + Debug,
    P: DateTimeProvider<Tz>,
{
    let maybe_preset =
        Preset::find_by_name_opt(m.value_of("BASE")).context(UtErrorKind::PresetError)?;
    let maybe_ymd =
        parse_argv_opt::<Ymd, YmdError>(m.value_of("YMD")).context(UtErrorKind::WrongDate)?;
    let maybe_hms =
        parse_argv_opt::<Hms, HmsError>(m.value_of("HMS")).context(UtErrorKind::WrongTime)?;
    let maybe_truncate =
        TimeUnit::find_by_name_opt(m.value_of("TRUNCATE")).context(UtErrorKind::TimeUnitError)?;

    let base = create_base_date(provider, maybe_preset, maybe_ymd, maybe_hms, maybe_truncate)?;
    let deltas = create_deltas(m.values_of("DELTA"))?;

    let maybe_precision = Precision::find_by_name_opt(m.value_of("PRECISION"))
        .context(UtErrorKind::PrecisionError)?;
    if maybe_precision.is_some() {
        eprintln!("-p PRECISION option is deprecated.");
    }
    let precision = maybe_precision.unwrap_or(precision);

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

fn create_base_date<P, Tz>(
    provider: P,
    maybe_preset: Option<Preset>,
    maybe_ymd: Option<Ymd>,
    maybe_hms: Option<Hms>,
    maybe_truncate: Option<TimeUnit>,
) -> Result<DateTime<Tz>, UtError>
where
    Tz: TimeZone + Debug,
    P: DateTimeProvider<Tz>,
{
    let now = provider.now();

    let maybe_date = maybe_preset
        .map(|p| Ok(Some(p.as_date(&provider))))
        .unwrap_or_else(|| {
            maybe_ymd.map_or(Ok(None), |ymd| {
                ymd.into_date(&provider.timezone()).map(Some)
            })
        })
        .context(UtErrorKind::WrongDate)?;

    let has_date = maybe_date.is_some();
    let date = maybe_date.unwrap_or_else(|| now.date());
    let time = maybe_hms.map(|hms| hms.into()).unwrap_or_else(|| {
        if has_date {
            NaiveTime::from_hms(0, 0, 0)
        } else {
            now.time()
        }
    });

    Ok(maybe_truncate
        .iter()
        .fold(date.and_time(time).unwrap(), |dt, unit| unit.truncate(dt)))
}

fn create_deltas(maybe_values: Option<Values>) -> Result<Vec<DeltaItem>, UtError> {
    let deltas = maybe_values
        .map(|values| {
            values
                .map(|s| DeltaItem::from_str(s).context(UtErrorKind::DeltaError))
                .collect()
        })
        .unwrap_or_else(|| Ok(Vec::new()))?;
    Ok(deltas)
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
