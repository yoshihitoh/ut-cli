use std::convert::TryFrom;
use std::fmt::Debug;
use std::str::FromStr;

use chrono::prelude::*;
use clap::ArgMatches;
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

#[derive(Debug)]
struct GenerateOptions {
    timestamp: Option<i64>,
    preset: Option<Preset>,
    ymd: Option<Ymd>,
    hms: Option<Hms>,
    truncate: Option<TimeUnit>,
    deltas: Vec<DeltaItem>,
}

impl GenerateOptions {
    pub fn base_datetime<P, Tz>(
        &self,
        provider: P,
        precision: Precision,
    ) -> Result<DateTime<Tz>, UtError>
    where
        Tz: TimeZone + Debug,
        P: DateTimeProvider<Tz>,
    {
        //
        let base = if let Some(timestamp) = self.timestamp {
            precision.parse_timestamp(provider.timezone(), timestamp)
        } else {
            let now = provider.now();
            let maybe_date = self.base_date(&provider)?;
            let has_date = maybe_date.is_some();
            let date = maybe_date.unwrap_or_else(|| now.date());
            let time = self.hms.map(|hms| hms.into()).unwrap_or_else(|| {
                if has_date {
                    NaiveTime::from_hms(0, 0, 0)
                } else {
                    now.time()
                }
            });

            date.and_time(time).unwrap()
        };

        Ok(self
            .truncate
            .iter()
            .fold(base, |dt, unit| unit.truncate(dt)))
    }

    fn base_date<P, Tz>(&self, provider: &P) -> Result<Option<Date<Tz>>, UtError>
    where
        Tz: TimeZone + Debug,
        P: DateTimeProvider<Tz>,
    {
        let date = self
            .preset
            .map(|p| Ok(Some(p.as_date(provider))))
            .unwrap_or_else(|| {
                self.ymd.map_or(Ok(None), |ymd| {
                    ymd.into_date(&provider.timezone()).map(Some)
                })
            })
            .context(UtErrorKind::WrongDate)?;

        Ok(date)
    }
}

impl TryFrom<&ArgMatches<'_>> for GenerateOptions {
    type Error = UtError;

    fn try_from(m: &ArgMatches<'_>) -> Result<Self, Self::Error> {
        fn delta_item_from(s: &str) -> Result<DeltaItem, UtError> {
            Ok(DeltaItem::from_str(s).context(UtErrorKind::DeltaError)?)
        }

        let timestamp = m
            .value_of("BASE_TIMESTAMP")
            .map(|s| {
                i64::from_str(s)
                    .map(Some)
                    .context(UtErrorKind::WrongTimestamp)
            })
            .unwrap_or_else(|| Ok(None))?;
        let preset =
            Preset::find_by_name_opt(m.value_of("BASE")).context(UtErrorKind::PresetError)?;
        let ymd =
            parse_argv_opt::<Ymd, YmdError>(m.value_of("YMD")).context(UtErrorKind::WrongDate)?;
        let hms =
            parse_argv_opt::<Hms, HmsError>(m.value_of("HMS")).context(UtErrorKind::WrongTime)?;
        let truncate = TimeUnit::find_by_name_opt(m.value_of("TRUNCATE"))
            .context(UtErrorKind::TimeUnitError)?;
        let deltas = m
            .values_of("DELTA")
            .map(|values| values.map(delta_item_from).collect())
            .unwrap_or_else(|| Ok(Vec::new()))?;

        Ok(GenerateOptions {
            timestamp,
            preset,
            ymd,
            hms,
            truncate,
            deltas,
        })
    }
}

pub struct GenerateRequest<Tz: TimeZone> {
    base: DateTime<Tz>,
    deltas: Vec<DeltaItem>,
    precision: Precision,
}

impl<Tz> GenerateRequest<Tz>
where
    Tz: TimeZone + Debug,
{
    pub fn new<P>(
        m: &ArgMatches,
        provider: P,
        precision: Precision,
    ) -> Result<GenerateRequest<Tz>, UtError>
    where
        P: DateTimeProvider<Tz>,
    {
        let maybe_precision = Precision::find_by_name_opt(m.value_of("PRECISION"))
            .context(UtErrorKind::PrecisionError)?;
        if maybe_precision.is_some() {
            eprintln!("-p PRECISION option is deprecated.");
        }
        let precision = maybe_precision.unwrap_or(precision);

        let generate_options = GenerateOptions::try_from(m)?;
        let base = generate_options.base_datetime(provider, precision)?;
        let deltas = generate_options.deltas;
        Ok(GenerateRequest {
            base,
            deltas,
            precision,
        })
    }
}

pub fn run<Tz>(request: GenerateRequest<Tz>) -> Result<(), UtError>
where
    Tz: TimeZone + Debug,
{
    generate(request)
}

fn generate<Tz: TimeZone>(request: GenerateRequest<Tz>) -> Result<(), UtError> {
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
