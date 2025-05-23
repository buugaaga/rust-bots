use crate::strategies::api_trait::Api;
use async_trait::async_trait;

pub struct BybitApi;

#[async_trait]
impl Api for BybitApi {
    async fn get_server_time(&self) -> Result<(), reqwest::Error> {
        let url = "https://api.bybit.com/v5/market/time";
        let response = reqwest::get(url).await?;
        let text = response.text().await?;
        println!("{}", text);

        Ok(())
    }
}

// pub async fn get_tickers() -> Result<(), reqwest::Error> {
//   let url = "https://api.bybit.com/v2/public/tickers";
//   let response = reqwest::get(url).await?;
//   let text = response.text().await?;
//   println!("{}", text);
//   Ok(())
// }
