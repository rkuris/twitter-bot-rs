#![warn(clippy::all, clippy::pedantic)]

use core::error::Error;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time;

mod config;
mod crawler;
mod twitter;

const CONF_FILENAME: &str = ".crypto-bot.conf";

fn get_home_dir() -> PathBuf {
    match home::home_dir() {
        Some(p) => p,
        None => {
            panic!("Impossible to get your home dir!");
        }
    }
}

#[allow(clippy::implicit_hasher)]
#[must_use]
pub fn build_message(prices: &HashMap<&str, crawler::CurrencyPrice>) -> String {
    let mut message = String::new();
    for (cur, value) in prices {
        message.push_str(&format!("#{}: ${} ${}€\n", cur, value.usd, value.eur));
    }
    message
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut conf_file_path: PathBuf = get_home_dir();
    conf_file_path.push(Path::new(CONF_FILENAME));

    let config = config::Config::read(&conf_file_path).map_err(|e| {
        eprintln!(
            "Error reading config file '{}': {}",
            conf_file_path.display(),
            e
        );
        e
    })?;

    let crawler = crawler::Crawler;

    let twitter = if let Some(consumer_key) = config.consumer_key {
        if let Some(consumer_secret) = config.consumer_secret {
            if let Some(access_key) = config.access_key {
                if let Some(access_secret) = config.access_secret {
                    Some(twitter::Twitter::new(
                        consumer_key,
                        consumer_secret,
                        access_key,
                        access_secret,
                    ))
                } else {
                    println!("Twitter access_secret is not set in the config file; Twitter will not be used");
                    None
                }
            } else {
                println!(
                    "Twitter access_key is not set in the config file; Twitter will not be used"
                );
                None
            }
        } else {
            println!(
                "Twitter consumer_secret is not set in the config file; Twitter will not be used"
            );
            None
        }
    } else {
        println!("Twitter consumer_key is not set in the config file; Twitter will not be used");
        None
    };

    // default delay (in seconds) is 10 minutes for if twitter is fully configured
    // otherwise it's 1 minute
    let delay = if twitter.is_some() { 600 } else { 60 };

    loop {
        let mut prices = HashMap::new();

        for currency in &config.currencies_to_follow {
            // Get the price of the currency
            let c = &currency[..];
            let price = crawler.get_price(c).await?;
            prices.insert(c, price);
        }

        let msg = build_message(&prices);
        if let Some(ref twitter) = twitter {
            twitter.tweet(&msg)?;
        } else {
            println!(
                "{}\n{}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S %Z"),
                msg
            );
        }
        tokio::time::sleep(time::Duration::from_secs(delay)).await;
    }
}
