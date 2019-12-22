use std::fmt::Debug;
use std::str::FromStr;

use regex::Match;

pub fn extract_number<E: Debug, T: FromStr<Err = E>>(maybe_match: Option<Match>) -> T {
    maybe_match
        .map(|m| m.as_str().parse().expect("must be a number text."))
        .unwrap()
}

pub fn parse_argv_opt<T, E>(maybe_text: Option<&str>) -> Result<Option<T>, E>
where
    T: FromStr<Err = E>,
{
    maybe_text
        .map(T::from_str)
        .map_or(Ok(None), |r| r.map(Some))
}
