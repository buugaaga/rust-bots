use config::Config;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub api: ApiSettings,
    pub logging: LoggingSettings,
}

#[derive(Debug, Deserialize)]
pub struct ApiSettings {
    // #[serde(rename = "BYBIT_API_KEY")]
    pub bybit_api_key: String,
    // #[serde(rename = "BYBIT_SECRET_KEY")]
    pub bybit_secret_key: String,
}

// #[derive(Debug, Deserialize)]
// pub struct Modules {
//     api: String,
//     strategies: String,
// }

#[derive(Debug, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    // pub format: String,
    // pub file_output: bool,
    // pub log_file: String,
    // pub modules: Modules,
}

impl LoggingSettings {
    // Get effective log filter, prioritizing environment variable
    pub fn get_log_filter(&self) -> String {
        // Check for RUST_LOG environment variable first
        env::var("RUST_LOG").unwrap_or_else(|_| {
            return self.level.clone();
        })
    }
}

pub fn get_settings() -> Settings {
    let settings = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()
        .unwrap();

    let bot_config = settings.try_deserialize().unwrap();

    return bot_config;
}
