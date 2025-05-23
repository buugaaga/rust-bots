mod api;
mod strategies;

use api::bybit::BybitApi;
use strategies::simple_strategy::SimpleStrategy;

#[tokio::main]
async fn main() {
    println!("Starting bot...");

    let api = BybitApi;
    let simple_strategy = SimpleStrategy;

    if let Err(err) = simple_strategy.run(&api).await {
        eprintln!("Error running strategy: {}", err);
    }
    // if let Err(err) = api::bybit::get_tickers().await {
    // strategies::simple_strategy::run_simple_strategy();
    //     eprintln!("Error fetching tickers: {}", err);
    // }
}
