use core::error::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

/// Configuration for twitter and currencies to follow
#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    /// Consumer key (aka API key) for Twitter
    pub consumer_key: Option<String>,

    /// Consumer secret (aka API secret) for Twitter
    pub consumer_secret: Option<String>,

    /// Access key for Twitter
    pub access_key: Option<String>,

    /// Access secret for Twitter
    pub access_secret: Option<String>,

    /// List of currencies to follow
    pub currencies_to_follow: Vec<String>,

    /// How often to poll for prices, in seconds
    pub interval_sec: Option<u64>,
}

impl Config {
    /// Read the config from a file
    pub fn read(path_file: &Path) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path_file)?;
        Ok(serde_json::from_reader(&mut file)?)
    }
}
