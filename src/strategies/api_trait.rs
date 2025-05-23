use async_trait::async_trait;
use reqwest::Error;

#[async_trait]
pub trait Api: Sync + Send {
    async fn get_server_time(&self) -> Result<(), Error>;
}
