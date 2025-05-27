mod api;
mod strategies;

use api::bybit::BybitApi;
use strategies::simple_strategy::SimpleStrategy;

const API_KEY: &'static str = "some_key";
const API_SECRET: &'static str = "some_secret";

#[tokio::main]
async fn main() {
    println!("Starting bot...");

    let api = BybitApi::new(API_KEY, API_SECRET);
    let simple_strategy = SimpleStrategy;

    if let Err(err) = simple_strategy.run(&api).await {
        eprintln!("Error running strategy: {}", err);
    }
}
