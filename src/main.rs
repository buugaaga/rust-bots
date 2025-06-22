mod api;
mod logging;
mod settings;
mod strategies;

use api::bybit::bybit::BybitApi;
use tracing::info;

use crate::strategies::divergence_strategy::DivergenceStrategy;

#[tokio::main]
async fn main() {
    let settings = settings::get_settings();

    logging::init_logging(&settings.logging);

    info!("Starting bot...: {}", "1,2,3");

    let bybit_api = BybitApi::new(&settings.api.bybit_api_key, &settings.api.bybit_secret_key);

    let divergence_strategy = DivergenceStrategy::new(bybit_api);

    divergence_strategy.run().await;
}
