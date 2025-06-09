mod api;
mod logging;
mod settings;
mod strategies;

use api::bybit::BybitApi;
use strategies::simple_strategy::SimpleStrategy;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    let settings = settings::get_settings();

    logging::init_logging(&settings.logging);
    info!("Starting bot...: {}", "1,2,3");

    let api = BybitApi::new(&settings.api.bybit_api_key, &settings.api.bybit_secret_key);
    let simple_strategy = SimpleStrategy;

    if let Err(err) = simple_strategy.run(&api).await {
        error!("Error running strategy: {}", err);
    }
}
