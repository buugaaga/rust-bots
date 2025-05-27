use async_trait::async_trait;
use reqwest::Error;

#[async_trait]
pub trait Api: Sync + Send {
    async fn get_server_time(&self) -> Result<(), Error>;

    async fn get_tickers(&self) -> Result<(), Error> {
        println!("That method is not implemented");
        Ok(())
    }

    // async fn get_candles(&self) -> &Vec<crate::api::bybit::Candlestick> {
    //     println!("That method is not implemented");
    //     &Vec::new()
    // }
}
