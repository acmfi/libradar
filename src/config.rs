use serde::Deserialize;
use std::error;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    lib_match_rate: Option<f32>,
    min_api_weight: Option<u32>,
    min_lib_count: Option<u32>,
    pkg_name_blacklist: Option<Vec<String>>,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn error::Error>> {
        todo!();
    }

    pub fn detect() -> Result<Config, Box<dyn error::Error>> {
        todo!();
    }
}
