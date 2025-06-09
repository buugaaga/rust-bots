use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::settings::LoggingSettings;

pub fn init_logging(settings: &LoggingSettings) {
    let env_filter = EnvFilter::new(settings.get_log_filter());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(env_filter)
        .init();
}
