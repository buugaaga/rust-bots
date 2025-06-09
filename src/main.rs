mod api;
mod settings;
mod strategies;

use api::bybit::BybitApi;
use strategies::simple_strategy::SimpleStrategy;

#[tokio::main]
async fn main() {
    println!("Starting bot...");

    let settings = settings::get_settings();

    let api = BybitApi::new(&settings.bybit_api_key, &settings.bybit_secret_key);
    let simple_strategy = SimpleStrategy;

    if let Err(err) = simple_strategy.run(&api).await {
        eprintln!("Error running strategy: {}", err);
    }
}
