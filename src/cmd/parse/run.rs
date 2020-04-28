use std::fmt::{Debug, Display};
use std::io;

use chrono::{Offset, TimeZone};
use clap::ArgMatches;
use failure::ResultExt;

use crate::error::{UtError, UtErrorKind};
use crate::find::FindByName;
use crate::precision::Precision;
use crate::provider::DateTimeProvider;
use crate::read::{read_next, ReadError};

#[derive(Debug)]
pub struct ParseRequest<P> {
    provider: P,
    precision: Precision,
    datetime_format: String,
    timestamp: i64,
}

impl<P> ParseRequest<P> {
    pub fn new(
        m: &ArgMatches,
        provider: P,
        precision: Precision,
        datetime_format: Option<&str>,
    ) -> Result<ParseRequest<P>, UtError> {
        let timestamp = get_timestamp(m.value_of("TIMESTAMP"))?;
        let maybe_precision = Precision::find_by_name_opt(m.value_of("PRECISION"))
            .context(UtErrorKind::PrecisionError)?;
        if maybe_precision.is_some() {
            eprintln!("-p PRECISION option is deprecated.");
        }
        let precision = maybe_precision.unwrap_or(precision);
        let datetime_format = datetime_format
            .unwrap_or_else(|| precision.preferred_format())
            .to_string();

        Ok(ParseRequest {
            provider,
            precision,
            datetime_format,
            timestamp,
        })
    }
}

pub fn run<O, Tz, P>(request: ParseRequest<P>) -> Result<(), UtError>
where
    O: Offset + Display + Sized,
    Tz: TimeZone<Offset = O> + Debug,
    P: DateTimeProvider<Tz>,
{
    let dt = request
        .precision
        .parse_timestamp(request.provider.timezone(), request.timestamp);
    println!("{}", dt.format(&request.datetime_format).to_string());
    Ok(())
}

fn get_timestamp(maybe_timestamp: Option<&str>) -> Result<i64, UtError> {
    Ok(maybe_timestamp
        .map(|s| s.parse::<i64>().context(UtErrorKind::WrongTimestamp))
        .unwrap_or_else(|| {
            let stdin = io::stdin();
            let r: Result<i64, ReadError> = read_next(stdin);
            r.context(UtErrorKind::WrongTimestamp)
        })?)
}
