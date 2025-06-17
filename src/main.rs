mod api;
mod logging;
mod settings;
mod strategies;

use api::bybit::BybitApi;
use strategies::simple_strategy::SimpleStrategy;
use tracing::{error, info};

use crate::api::exchange::Exchang;

#[tokio::main]
async fn main() {
    let settings = settings::get_settings();

    logging::init_logging(&settings.logging);

    info!("Starting bot...: {}", "1,2,3");

    let api = BybitApi::new(&settings.api.bybit_api_key, &settings.api.bybit_secret_key);
    let simple_strategy = SimpleStrategy;

    let symbol = "BTCUSDT";
    let interval = "1h";
    let limit: u32 = 300;

    let klines = api.fetch_klines(&symbol, &interval, limit).await;
    // let klines = api::bybit::BybitApi::fetch_klines(&symbol, &interval, &limit);

    // println!("klines {:#?}", klines);
    info!("klines: {:#?}", klines);
    // if let Err(err) = simple_strategy.run(&api).await {
    //     error!("Error running strategy: {}", err);
    // }
}
