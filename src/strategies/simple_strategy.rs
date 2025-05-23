// pub async fn run_simple_strategy() {
//     api::bybit::get_time_server();
// }
use crate::strategies::api_trait::Api;

pub struct SimpleStrategy;

impl SimpleStrategy {
    pub async fn run<T: Api + Sync + Send>(&self, api: &T) -> Result<(), reqwest::Error> {
        api.get_server_time().await?;
        println!("Trading strategy ran successfully.");
        Ok(())
    }
}
