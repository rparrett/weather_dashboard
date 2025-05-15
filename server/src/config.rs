use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub listen_addr: String,
    pub pirate_weather_key: String,
    pub locations: Vec<Location>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0:3000".to_string(),
            pirate_weather_key: Default::default(),
            locations: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub link: Option<String>,
}

impl Config {
    pub fn new(path: impl AsRef<Path>) -> Result<Config> {
        let input = fs::read_to_string(path)?;

        let config = toml::from_str(&input)?;

        Ok(config)
    }
}
