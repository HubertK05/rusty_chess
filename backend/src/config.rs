use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct AppSettings {
    pub eval_print: bool,
    pub pruning: bool,
    pub positional_value_factor: i32,
    pub search_depth: u8,
}

impl AppSettings {
    pub fn get_from_file(path: &str) -> Result<Self, ConfigError> {
        let config_source = Config::builder()
            .add_source(config::File::with_name(path))
            .build()?;

        let mut settings = config_source.try_deserialize::<AppSettings>()?;

        settings.positional_value_factor = settings.positional_value_factor.clamp(0, 100);
        settings.search_depth = settings.search_depth.max(1);
        Ok(settings)
    }
}
