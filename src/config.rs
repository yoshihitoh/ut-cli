use std::env;

#[derive(Debug)]
pub struct Config {
    offset: Option<String>,
    precision: Option<String>,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            offset: env::var("UT_OFFSET").ok(),
            precision: env::var("UT_PRECISION").ok(),
        }
    }

    pub fn offset(&self) -> Option<&str> {
        self.offset.as_deref()
    }

    pub fn precision(&self) -> Option<&str> {
        self.precision.as_deref()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            offset: None,
            precision: None,
        }
    }
}
