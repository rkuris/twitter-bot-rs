use core::error::Error;
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CurrencyPrice {
    pub usd: f64,
    pub eur: f64,
}
#[derive(Debug, Default)]
pub struct Crawler;

impl Crawler {
    fn parse_content(content: &str) -> CurrencyPrice {
        serde_json::from_str(content).unwrap()
    }

    async fn http_get(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let url = url.parse::<hyper::Uri>().unwrap();

        let https = HttpsConnector::new();
        let client = Client::builder(TokioExecutor::new()).build::<_, Empty<Bytes>>(https);

        let res = client.get(url).await?;
        assert_eq!(res.status(), 200);
        let body = res.into_body();
        let body = body.collect().await?.to_bytes();
        Ok(String::from_utf8_lossy(&body).to_string())
    }

    pub async fn get_price(&self, currency: &str) -> Result<CurrencyPrice, Box<dyn Error>> {
        let url =
            format!("https://min-api.cryptocompare.com/data/price?fsym={currency}&tsyms=USD,EUR");

        let content = self.http_get(&url[..]).await?;
        let values: CurrencyPrice = Crawler::parse_content(&content);
        Ok(values)
    }
}
