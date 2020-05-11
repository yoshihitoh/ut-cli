use std::io::{self, Read};
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("IO error. error:{0}")]
    Io(io::Error),

    #[error("Parse int error. error:{0}")]
    ParseInt(ParseIntError),
}

impl From<io::Error> for ReadError {
    fn from(e: io::Error) -> Self {
        ReadError::Io(e)
    }
}

impl From<ParseIntError> for ReadError {
    fn from(e: ParseIntError) -> Self {
        ReadError::ParseInt(e)
    }
}

pub fn read_next<R, T, E>(src: R) -> Result<T, ReadError>
where
    R: Read,
    T: FromStr<Err = E>,
    E: Into<ReadError>,
{
    let s: String = src
        .bytes()
        .map(|r| r.map(|b| b as char))
        .skip_while(|r| {
            r.as_ref()
                .map(|c| c.is_whitespace())
                .unwrap_or_else(|_| false)
        })
        .take_while(|r| {
            r.as_ref()
                .map(|c| !c.is_whitespace())
                .unwrap_or_else(|_| true)
        })
        .collect::<Result<_, _>>()?;

    s.parse().map_err(|e: E| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_basics() {
        let r: Result<i64, ReadError> = read_next("12345".as_bytes());
        assert_eq!(Some(12345), r.ok());

        let r: Result<i64, ReadError> = read_next(" 12345".as_bytes());
        assert_eq!(Some(12345), r.ok());

        let r: Result<i64, ReadError> = read_next("12345 ".as_bytes());
        assert_eq!(Some(12345), r.ok());

        let r: Result<i64, ReadError> = read_next(" 12345 ".as_bytes());
        assert_eq!(Some(12345), r.ok());

        let r: Result<i64, ReadError> = read_next(" 11111 22222 ".as_bytes());
        assert_eq!(Some(11111), r.ok());
    }
}
