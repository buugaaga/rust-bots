use crate::api::exchange::{Exchang, Kline};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

use tracing::{error, info};

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

// #[allow(dead_code)]
// #[derive(Debug, Deserialize)]
// struct TickersResult {
//     category: String,
//     list: Vec<Ticker>,
// }

#[derive(Debug, Deserialize)]
struct BybitKlineResult {
    // category: String,
    // symbol: String,
    list: Vec<Vec<String>>,
}

// #[allow(dead_code)]
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct Ticker {
//     symbol: String,
//     bid1_price: String,
//     bid1_size: String,
//     ask1_price: String,
//     ask1_size: String,
//     last_price: String,
//     prev_price_24h: String,
//     price_24h_pcnt: String,
//     high_price_24h: String,
//     low_price_24h: String,
//     turnover_24h: String,
//     volume_24h: String,
// }

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
        let url = Self::build_url("/market/time");
        let response = reqwest::get(url).await?;
        let text = response.text().await?;
        println!("server time: {}", text);

        Ok(())
    }

    async fn fetch_klines(&self, symbol: &str, interval: &str, limit: u32) -> Result<Vec<Kline>> {
        let endpoint = format!(
            "/market/kline?category=spot&symbol={}&interval={}&limit={}",
            symbol, interval, limit
        );
        let url = Self::build_url(&endpoint);
        info!("url {}", url);
        let response = reqwest::get(url).await?;

        if !response.status().is_success() {
            error!("Error: API returned status {}", response.status());
            return Ok(vec![]); // or handle the error differently
        }

        let data: BybitResponse<BybitKlineResult> = response.json().await?;

        let mut result: Vec<Kline> = Vec::new();

        for item in data.result.list {
            if item.len() < 7 {
                error!("Bybit item list must be more than 7");
                return Ok(vec![]);
            }
            let timestamp = item[0].parse::<i64>()?;
            let open = item[1].parse::<f64>()?;
            let high = item[2].parse::<f64>()?;
            let low = item[3].parse::<f64>()?;
            let close = item[4].parse::<f64>()?;
            let volume = item[5].parse::<f64>()?;

            result.push(Kline {
                timestamp,
                open,
                close,
                low,
                high,
                volume,
            })
        }

        info!("result {:#?}", &result[1..3]);

        return Ok(result);
    }
}
