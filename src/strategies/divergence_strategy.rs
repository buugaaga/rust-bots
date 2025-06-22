use std::time::Duration;

use crate::api::exchange::Exchange;
use tokio::time::interval;
use tracing::{error, info};

pub struct DivergenceStrategy<T: Exchange> {
    exchange: T,
}

impl<T: Exchange> DivergenceStrategy<T> {
    pub fn new(exchange: T) -> Self {
        return Self { exchange };
    }
    // Calculate RSI values
    // fn calculate_rsi(&self) -> Vec<f64> {
    //     // Use a crate like `ta` or implement RSI logic
    //     // Example: https://docs.rs/ta/latest/ta/
    //     todo!()
    // }

    // Detect bullish divergence (price LL vs RSI HL)
    // fn find_bullish_divergence(&self) -> bool {
    //     // let rsi = self.calculate_rsi();
    //     // let last_n = 3; // Check last 3 candles

    //     // // Check if price has lower lows
    //     // let price_lows = self.candles.iter().rev().take(last_n).map(|c| c.low);
    //     // let is_price_ll = is_descending(price_lows);

    //     // // Check if RSI has higher lows
    //     // let rsi_lows = rsi.iter().rev().take(last_n).copied();
    //     // let is_rsi_hl = is_ascending(rsi_lows);

    //     // is_price_ll && is_rsi_hl
    //     todo!()
    // }

    pub async fn run(&self) {
        let symbol = "BTCUSDT";
        let kline_interval = "30";
        let limit: u32 = 300;

        let mut timer = interval(Duration::from_secs(30));

        loop {
            timer.tick().await;

            match self
                .exchange
                .fetch_klines(&symbol, &kline_interval, limit)
                .await
            {
                Ok(klines) => info!("klines in run {:?}", &klines[0..3]),
                Err(err) => error!("Ошибка ёпта!!!: {}", err),
            }
        }
    }

    // async fn fetch_klines_periodically(&self, period: u32) {
    //     // let fetch_klines_interval = interval(period);
    //     todo!()
    //     // loop {
    //     //     select! {

    //     //     }
    //     // }
    // }
}
