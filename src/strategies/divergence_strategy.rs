struct DivergenceStrategy {
    rsi_period: usize, // e.g., RSI(14)
    candles: Vec<Candlestick>,
}

impl DivergenceStrategy {
    // Calculate RSI values
    fn calculate_rsi(&self) -> Vec<f64> {
        // Use a crate like `ta` or implement RSI logic
        // Example: https://docs.rs/ta/latest/ta/
        todo!()
    }

    // Detect bullish divergence (price LL vs RSI HL)
    fn find_bullish_divergence(&self) -> bool {
        let rsi = self.calculate_rsi();
        let last_n = 3; // Check last 3 candles

        // Check if price has lower lows
        let price_lows = self.candles.iter().rev().take(last_n).map(|c| c.low);
        let is_price_ll = is_descending(price_lows);

        // Check if RSI has higher lows
        let rsi_lows = rsi.iter().rev().take(last_n).copied();
        let is_rsi_hl = is_ascending(rsi_lows);

        is_price_ll && is_rsi_hl
    }
}
