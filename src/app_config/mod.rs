use crate::{
    app_config::providers::{ProviderConfig, local::LocalConfig},
    utils::get_config_file::get_config_file,
};
use color_eyre::{Result, eyre};
use config::{Config, File};
use serde::Deserialize;
use std::path::PathBuf;

pub mod providers;

#[derive(Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_providers")]
    pub providers: Vec<ProviderConfig>,
}

fn default_providers() -> Vec<ProviderConfig> {
    vec![ProviderConfig::Local {
        config: LocalConfig {
            path: PathBuf::from("~/Music"),
        },
    }]
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let mut builder = Config::builder();

        let config_path = get_config_file()?;

        if let Some(config_path) = config_path {
            builder = builder.add_source(File::from(config_path));
        }

        builder = builder.add_source(config::Environment::with_prefix("TUISIC"));

        let settings = builder.build().map_err(|e| eyre::eyre!(e))?;
        settings.try_deserialize().map_err(|e| eyre::eyre!(e))
    }
}
