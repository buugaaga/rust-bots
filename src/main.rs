mod api;
mod logging;
mod settings;
mod strategies;

use api::bybit::BybitApi;
use strategies::simple_strategy::SimpleStrategy;
use tracing::{error, info};

use crate::{api::exchange::Exchange, strategies::divergence_strategy::DivergenceStrategy};

#[tokio::main]
async fn main() {
    let settings = settings::get_settings();

    logging::init_logging(&settings.logging);

    info!("Starting bot...: {}", "1,2,3");

    let bybit_api = BybitApi::new(&settings.api.bybit_api_key, &settings.api.bybit_secret_key);

    let divergence_strategy = DivergenceStrategy::new(bybit_api);

    divergence_strategy.run().await;
    // let simple_strategy = SimpleStrategy;

    // let symbol = "BTCUSDT";
    // let interval = "60";
    // let limit: u32 = 300;

    // let klines = api.fetch_klines(&symbol, &interval, limit).await;
    // let klines = api::bybit::BybitApi::fetch_klines(&symbol, &interval, &limit);

    // println!("klines {:#?}", klines);
    // info!("main klines: {:#?}", klines);
    // if let Err(err) = simple_strategy.run(&api).await {
    //     error!("Error running strategy: {}", err);
    // }
}
