use std::convert::TryFrom;
use std::str::FromStr;

use chrono::{Date, DateTime, Local, LocalResult, NaiveTime, TimeZone, Utc};
use clap::{ArgMatches, Values};
use failure::{Fail, ResultExt};

use crate::delta::DeltaItem;
use crate::error::{UtError, UtErrorKind};
use crate::precision::Precision;
use crate::preset::{DateFixture, LocalDateFixture, Preset, UtcDateFixture};
use crate::unit::TimeUnit;

pub struct Request<Tz: TimeZone> {
    base: DateTime<Tz>,
    deltas: Vec<DeltaItem>,
    precision: Precision,
}

impl<Tz: TimeZone> Request<Tz> {
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

impl TryFrom<&ArgMatches<'static>> for Request<Utc> {
    type Error = UtError;

    fn try_from(m: &ArgMatches) -> Result<Self, Self::Error> {
        parse(m, UtcDateFixture {})
    }
}

impl TryFrom<&ArgMatches<'static>> for Request<Local> {
    type Error = UtError;

    fn try_from(m: &ArgMatches) -> Result<Self, Self::Error> {
        parse(m, LocalDateFixture {})
    }
}

fn parse<Tz, F>(m: &ArgMatches, fixture: F) -> Result<Request<Tz>, UtError>
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
    let precision = parse_precision(m.value_of("PRECISION"))?;

    Ok(Request {
        base,
        deltas,
        precision,
    })
}

fn extract_int(s: &str, start: usize, stop: usize) -> i32 {
    *&s[start..stop].parse().expect("not a number")
}

fn parse_ymd<Tz: TimeZone>(tz: Tz, ymd: &str) -> Result<Date<Tz>, UtError> {
    let year_len = ymd.len() - 4;
    let y = extract_int(ymd, 0, year_len);
    let m = extract_int(ymd, year_len, year_len + 2);
    let d = extract_int(ymd, year_len + 2, year_len + 4);

    match tz.ymd_opt(y, m as u32, d as u32) {
        LocalResult::Single(date) => Ok(date),
        LocalResult::None => Err(UtError::from(UtErrorKind::WrongDate)),
        LocalResult::Ambiguous(_, _) => Err(UtError::from(UtErrorKind::AmbiguousDate)),
    }
}

fn parse_hms(hms: &str) -> NaiveTime {
    let h = *&hms[0..2].parse::<u32>().expect("not a number");
    let m = *&hms[2..4].parse::<u32>().expect("not a number");
    let s = *&hms[4..6].parse::<u32>().expect("not a number");

    NaiveTime::from_hms(h, m, s)
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
    let maybe_date = maybe_base
        .map(|s| {
            Preset::find_by_name(s)
                .map(|p| p.as_date(&fixture))
                .context(UtErrorKind::PresetError)
                .map_err(UtError::from)
        })
        .or_else(|| maybe_ymd.map(|ymd| parse_ymd(fixture.timezone(), ymd)));

    // time (hms)
    let maybe_time = maybe_hms.map(parse_hms);

    // datetime
    let dt = maybe_date
        .map(|date| {
            date.map(|d| {
                maybe_time
                    .map(|t| d.and_time(t).expect("not a datetime"))
                    .unwrap_or(d.and_hms(0, 0, 0))
            })
        })
        .unwrap_or_else(|| {
            Ok(maybe_time
                .map(|t| now.date().and_time(t).unwrap())
                .unwrap_or(now))
        })?;

    // truncate
    Ok(if let Some(truncate) = maybe_truncate {
        let truncate_unit = TimeUnit::find_by_name(truncate).context(UtErrorKind::TimeUnitError)?;
        truncate_unit.truncate(dt)
    } else {
        dt
    })
}

fn parse_deltas(maybe_values: Option<Values>) -> Result<Vec<DeltaItem>, UtError> {
    maybe_values
        .map(|values| {
            values
                .map(|v| {
                    DeltaItem::from_str(v)
                        .context(UtErrorKind::DeltaError)
                        .map_err(UtError::from)
                })
                .collect()
        })
        .unwrap_or(Ok(Vec::new()))
}

fn parse_precision(maybe_precision: Option<&str>) -> Result<Precision, UtError> {
    maybe_precision
        .map(|p| {
            Precision::find_by_name(p)
                .map_err(|e| e.context(UtErrorKind::PrecisionError))
                .map_err(UtError::from)
        })
        .unwrap_or(Ok(Precision::Second))
}
