use std::env;

static OFFSET_KEY: &'static str = "UT_OFFSET";
static PRECISION_KEY: &'static str = "UT_PRECISION";

#[derive(Debug)]
pub struct Config {
    offset: Option<String>,
    precision: Option<String>,
}

impl Config {
    pub fn from_env() -> Config {
        Config {
            offset: env::var(OFFSET_KEY).ok(),
            precision: env::var(PRECISION_KEY).ok(),
        }
    }

    pub fn offset(&self) -> Option<&str> {
        self.offset.as_ref().map(|s| s.as_str())
    }

    pub fn precision(&self) -> Option<&str> {
        self.precision.as_ref().map(|s| s.as_str())
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
