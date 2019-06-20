use std::convert::TryFrom;
use std::str::FromStr;

use chrono::{Date, DateTime, Local, NaiveTime, TimeZone, Utc};
use clap::{ArgMatches, Values};
use failure::ResultExt;

use crate::argv::{HmsArgv, ParseArgv, PrecisionArgv, PresetArgv, TimeUnitArgv, YmdArgv};
use crate::delta::DeltaItem;
use crate::error::{UtError, UtErrorKind};
use crate::precision::Precision;
use crate::preset::{DateFixture, LocalDateFixture, UtcDateFixture};

pub struct OldRequest<Tz: TimeZone> {
    base: DateTime<Tz>,
    deltas: Vec<DeltaItem>,
    precision: Precision,
}

impl<Tz: TimeZone> OldRequest<Tz> {
    pub fn base(&self) -> DateTime<Tz> {
        self.base.clone()
    }

    pub fn deltas(&self) -> &[DeltaItem] {
        &self.deltas
    }

    pub fn precision(&self) -> Precision {
        self.precision
    }
}

impl TryFrom<&ArgMatches<'static>> for OldRequest<Utc> {
    type Error = UtError;

    fn try_from(m: &ArgMatches) -> Result<Self, Self::Error> {
        parse_request(m, UtcDateFixture {})
    }
}

impl TryFrom<&ArgMatches<'static>> for OldRequest<Local> {
    type Error = UtError;

    fn try_from(m: &ArgMatches) -> Result<Self, Self::Error> {
        parse_request(m, LocalDateFixture {})
    }
}

fn parse_request<Tz, F>(m: &ArgMatches, fixture: F) -> Result<OldRequest<Tz>, UtError>
where
    Tz: TimeZone,
    F: DateFixture<Tz>,
{
    let base = parse_base(
        fixture,
        m.value_of("BASE"),
        m.value_of("YMD"),
        m.value_of("HMS"),
        m.value_of("TRUNCATE"),
    )?;
    let deltas = parse_deltas(m.values_of("DELTA"))?;

    let precision_argv = PrecisionArgv::default();
    let precision = m
        .value_of("PRECISION")
        .map(|s| precision_argv.parse_argv(s))
        .unwrap_or(Ok(Precision::Second))?;

    Ok(OldRequest {
        base,
        deltas,
        precision,
    })
}

fn parse_base<F, Tz>(
    fixture: F,
    maybe_base: Option<&str>,
    maybe_ymd: Option<&str>,
    maybe_hms: Option<&str>,
    maybe_truncate: Option<&str>,
) -> Result<DateTime<Tz>, UtError>
where
    F: DateFixture<Tz>,
    Tz: TimeZone,
{
    let now = fixture.now();

    // date (preset => ymd)
    let ymd_argv = YmdArgv::from(fixture.timezone());
    let preset_argv = PresetArgv::default();
    let maybe_date = maybe_base
        .map(|s| preset_argv.parse_argv(s))
        .map(|r| r.map(|p| p.as_date(&fixture)))
        .or_else(|| maybe_ymd.map(|ymd| ymd_argv.parse_argv(ymd)));

    // time (hms)
    let hms_argv = HmsArgv::default();
    let maybe_time = maybe_hms.map(|hms| hms_argv.parse_argv(hms));

    // datetime
    let has_date = maybe_date.is_some();
    let d: Date<Tz> = maybe_date.unwrap_or(Ok(now.date()))?;
    let t: NaiveTime = maybe_time.unwrap_or(Ok(if has_date {
        NaiveTime::from_hms(0, 0, 0)
    } else {
        now.time()
    }))?;
    let dt = d.and_time(t).unwrap();

    // truncate
    Ok(if let Some(truncate) = maybe_truncate {
        let timeunit_argv = TimeUnitArgv::default();
        let truncate_unit = timeunit_argv.parse_argv(truncate)?;
        truncate_unit.truncate(dt)
    } else {
        dt
    })
}

fn parse_deltas(maybe_values: Option<Values>) -> Result<Vec<DeltaItem>, UtError> {
    fn parse(s: &str) -> Result<DeltaItem, UtError> {
        DeltaItem::from_str(s)
            .context(UtErrorKind::DeltaError)
            .map_err(UtError::from)
    }

    maybe_values
        .map(|values| values.map(parse).collect())
        .unwrap_or(Ok(Vec::new()))
}
