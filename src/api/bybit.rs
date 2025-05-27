use crate::strategies::api_trait::Api;
use async_trait::async_trait;
use serde::Deserialize;

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
struct Candlestick {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    timestamp: u64, // 5-minute interval
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

    fn build_url(endpoing: &str) -> String {
        return format!("{}{}", Self::BASE_URL, endpoing.to_string());
    }
}

#[async_trait]
impl Api for BybitApi {
    async fn get_server_time(&self) -> Result<(), reqwest::Error> {
        let url = "https://api.bybit.com/v5/market/time";
        let response = reqwest::get(url).await?;
        let text = response.text().await?;
        println!("server time: {}", text);

        Ok(())
    }

    async fn get_tickers(&self) -> Result<(), reqwest::Error> {
        let url = Self::build_url("/market/tickers?category=spot&symbol=ATHUSDT");
        let response = reqwest::get(url).await?;

        // Check if the request was successful
        if !response.status().is_success() {
            eprintln!("Error: API returned status {}", response.status());
            return Ok(()); // or handle the error differently
        }

        let json = response.json::<BybitResponse<TickersResult>>().await?;
        // let text = response.text().await?;

        let tickers = json;
        println!("tickers text {:#?}", tickers);

        Ok(())
    }

    // async fn get_candles(&self) -> &Vec<Candlestick> {}
}
