use crate::api::{
    bybit::utils,
    exchange::{CreateOrderResult, CreateOrderSide, Exchange, ExchangeError, Kline},
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

use serde_json::json;
use tracing::error;

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

    fn build_private_header(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            "X-BAPI-API-KEY",
            HeaderValue::from_str(&self.api_key).unwrap(),
        );
        headers.insert(
            "X-BAPI-SING",
            HeaderValue::from_str(&self.api_secret).unwrap(),
        );

        return headers;
    }
}

#[async_trait]
impl Exchange for BybitApi {
    // async fn fetch_server_time(&self) -> Result<()> {
    //     let url = Self::build_url("/market/time");
    //     let response = reqwest::get(url).await?;
    //     let text = response.text().await?;
    //     println!("server time: {}", text);

    //     Ok(())
    // }

    async fn fetch_klines(&self, symbol: &str, interval: &str, limit: u32) -> Result<Vec<Kline>> {
        let endpoint = format!(
            "/market/kline?category=spot&symbol={}&interval={}&limit={}",
            symbol, interval, limit
        );
        let url = Self::build_url(&endpoint);

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

        return Ok(result);
    }

    async fn create_limit_order(
        &self,
        symbol: &str,
        side: CreateOrderSide,
        qty: &str,
        price: &str,
        take_profit: &str,
        stop_loss: &str,
    ) -> Result<CreateOrderResult, ExchangeError> {
        let client = reqwest::Client::new();

        let endpoint_path = "/order/create".to_string();

        let mut body = serde_json::Map::new();

        body.insert("category".to_string(), json!("spot"));
        body.insert("symbol".to_string(), json!(symbol));
        body.insert("side".to_string(), json!(side.to_string()));
        body.insert("qty".to_string(), json!(qty));
        body.insert("price".to_string(), json!(price));
        body.insert("takeProfit".to_string(), json!(take_profit));
        body.insert("stopLoss".to_string(), json!(stop_loss));

        let timestamp = Utc::now().timestamp_millis().to_string();
        let recv_window = "5000";

        let signature = match utils::generate_post_signature(
            &self.api_key,
            &self.api_secret,
            &body,
            &timestamp,
            &recv_window,
        ) {
            Ok(s) => s,
            Err(e) => return Err(ExchangeError::new("generate_post_signature error", 500)),
        };

        let url = Self::build_url(&endpoint_path);
        let res = client
            .post(url)
            .json(&body)
            // .body(&body)
            .header("X-BAPI-API-KEY", self.api_key)
            .header("X-BAPI-SING", signature)
            .header("Context-Type", "application/json")
            .header("X-BAPI-TIMESTAMP", timestamp)
            .header("X-BAPI-RECV-WINDOW", recv_window)
            .send();
        todo!()
    }
}
