use config::Config;

use std::collections::HashMap;

pub fn get_config() -> HashMap<String, String> {
    let settings = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()
        .unwrap();
    let hash_map = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();
    return hash_map;
}
