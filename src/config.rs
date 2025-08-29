use std::env;

#[derive(Debug, Default)]
pub struct Config {
    offset: Option<String>,
    precision: Option<String>,
    datetime_format: Option<String>,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            offset: env::var("UT_OFFSET").ok(),
            precision: env::var("UT_PRECISION").ok(),
            datetime_format: env::var("UT_DATETIME_FORMAT").ok(),
        }
    }

    pub fn offset(&self) -> Option<&str> {
        self.offset.as_deref()
    }

    pub fn precision(&self) -> Option<&str> {
        self.precision.as_deref()
    }

    pub fn datetime_format(&self) -> Option<&str> {
        self.datetime_format.as_deref()
    }
}
