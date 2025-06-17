use crate::api::exchange::{Exchang, Kline};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json;
use tracing::info;

// Define the data structures for the Bybit tickers response
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BybitResponse<T> {
    ret_code: i32,
    ret_msg: String,
    result: T,
    time: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct TickersResult {
    category: String,
    list: Vec<Ticker>,
}

#[derive(Debug, Deserialize)]
struct BybitKlineResult {
    category: String,
    symbol: String,
    list: Vec<BybitKline>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Ticker {
    symbol: String,
    bid1_price: String,
    bid1_size: String,
    ask1_price: String,
    ask1_size: String,
    last_price: String,
    prev_price_24h: String,
    price_24h_pcnt: String,
    high_price_24h: String,
    low_price_24h: String,
    turnover_24h: String,
    volume_24h: String,
    // index_price: String,
    //
    //
    // Add other fields as needed
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BybitKline {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    timestamp: i64, // 5-minute interval
}

pub struct BybitApi {
    api_key: String,
    api_secret: String,
}

impl BybitApi {
    const BASE_URL: &'static str = "https://api.bybit.com/v5";

    pub fn new(api_key: &str, api_secret: &str) -> Self {
        BybitApi {
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
        }
    }

    fn build_url(endpoint: &str) -> String {
        return format!("{}{}", Self::BASE_URL, endpoint.to_string());
    }
}

#[async_trait]
impl Exchang for BybitApi {
    async fn fetch_server_time(&self) -> Result<()> {
        // let url = "https://api.bybit.com/v5/market/time";
        let url = Self::build_url("/market/time");
        let response = reqwest::get(url).await?;
        let text = response.text().await?;
        println!("server time: {}", text);

        Ok(())
    }

    async fn fetch_klines(&self, symbol: &str, interval: &str, limit: u32) -> Result<Vec<Kline>> {
        let endpoint = format!(
            "/market/kline?category=inverse&symbol={}&interval={}&limit={}",
            symbol, interval, limit
        );
        let url = Self::build_url(&endpoint);
        let response = reqwest::get(url).await?;

        // Check if the request was successful
        if !response.status().is_success() {
            eprintln!("Error: API returned status {}", response.status());
            return Ok(vec![]); // or handle the error differently
        }

        // let json = response.json::<BybitResponse<BybitKlineResult>>().await?;
        // let tickers = json;
        // info!("tickers {:#?}", tickers);
        let data: serde_json::Value = response.json().await?;

        info!("raw data {:#?}", data);

        let klines: Vec<BybitKline> = serde_json::from_value(data["result"].clone())?;

        let result: Vec<Kline> = klines
            .iter()
            .map(|k| Kline {
                open: k.open,
                close: k.close,
                low: k.low,
                high: k.high,
                volume: k.volume,
                timestamp: k.timestamp,
            })
            .collect();
        return Ok(result);
    }
}
