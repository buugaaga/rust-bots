use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BotConfig {
    #[serde(rename = "BYBIT_API_KEY")]
    pub bybit_api_key: String,
    #[serde(rename = "BYBIT_SECRET_KEY")]
    pub bybit_secret_key: String,
}

pub fn get_settings() -> BotConfig {
    let settings = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()
        .unwrap();

    let bot_config = settings.try_deserialize().unwrap();

    return bot_config;
}
