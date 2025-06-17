use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct Kline {
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
    pub volume: f64,
    pub timestamp: i64,
}

#[async_trait]
pub trait Exchang: Sync + Send {
    async fn fetch_server_time(&self) -> Result<()>;

    async fn fetch_klines(&self, symbol: &str, interval: &str, limit: u32) -> Result<Vec<Kline>>;

    // pub fn create_order(&self, symbol: &str, )
}
