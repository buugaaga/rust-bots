use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum_macros::Display as EnumDisplay;

use crate::api::exchange_error::ExchangeError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Kline {
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
    pub volume: f64,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrderResult {
    pub order_id: String,
}

#[derive(EnumDisplay)]
pub enum CreateOrderSide {
    Buy,
    Sell,
}

#[derive(EnumDisplay, Deserialize, Serialize)]
pub enum OrderType {
    Market,
    Limit,
}

/// Exhange trait
#[async_trait]
pub trait Exchange: Sync + Send {
    // async fn fetch_server_time(&self) -> Result<()>;

    // fetch klines
    async fn fetch_klines(&self, symbol: &str, interval: &str, limit: u32) -> Result<Vec<Kline>>;

    /// Creates a new order on the exchange.
    /// # Parameters
    /// * symbol - Trading pair symbol (e.g., "BTCUSDT")
    /// * side - CreateOrderSide
    /// * qty - Order quantity (amount of base asset to buy or sell), e.g "0.01"
    /// * price - Order price, e.g. "15600"
    /// * take_profit - Take profit price, e.g. "15700"
    /// * stop_loss - Stop loss price, e.g. "15500"
    async fn create_limit_order(
        &self,
        symbol: &str,
        side: CreateOrderSide,
        qty: &str,
        price: &str,
        take_profit: &str,
        stop_loss: &str,
    ) -> Result<CreateOrderResult, ExchangeError>;
}
