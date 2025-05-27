use crate::strategies::api_trait::Api;
use tokio::select;
use tokio::time::{Duration, interval};

pub struct SimpleStrategy;

impl SimpleStrategy {
    pub async fn run<T: Api + Sync + Send>(&self, api: &T) -> Result<(), reqwest::Error> {
        let mut server_time_interval = interval(Duration::from_secs(15));

        let mut tickets_interval = interval(Duration::from_secs(5));

        server_time_interval.tick().await;
        tickets_interval.tick().await;

        loop {
            select! {
                _ = server_time_interval.tick() => {
                    match api.get_server_time().await {
                        Ok(_) => println!("Succes fetched server time"),
                        Err(e) => eprintln!("Error fetching server time: {}", e),
                    }
                }

                  _ = tickets_interval.tick() => {
                    match api.get_tickers().await {
                        Ok(_) => println!("Successfully fetched tickers"),
                        Err(e) => eprintln!("Error fetching tickers: {}", e),
                    }
                }
            }
            // server_time_interval.tick().await;

            // match api.get_server_time().await {
            //     Ok(_) => println!("Trading strategy ran successfully."),
            //     Err(e) => println!("Error in simple_strategy {}", e),
            // }
        }
    }
}
