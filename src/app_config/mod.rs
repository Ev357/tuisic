use crate::app_config::providers::{ProviderConfig, local::LocalConfig};
use color_eyre::{Result, eyre};
use config::{Config, File};
use directories::ProjectDirs;
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

        let config_path = Self::get_config_file()?;

        if let Some(config_path) = config_path {
            builder = builder.add_source(File::from(config_path));
        }

        builder = builder.add_source(config::Environment::with_prefix("TUISIC"));

        let settings = builder.build().map_err(|e| eyre::eyre!(e))?;
        settings.try_deserialize().map_err(|e| eyre::eyre!(e))
    }

    pub fn get_config_file() -> Result<Option<PathBuf>> {
        let directory = Self::get_config_dir()?;

        let config_file = directory.join("config.toml");

        if !config_file.exists() {
            return Ok(None);
        }

        Ok(Some(config_file))
    }

    fn get_config_dir() -> Result<PathBuf> {
        let directory = if let Ok(s) = std::env::var("TUISIC_CONFIG") {
            PathBuf::from(s)
        } else if let Some(proj_dirs) = ProjectDirs::from("dev", "evest", "tuisic") {
            proj_dirs.config_local_dir().to_path_buf()
        } else {
            return Err(eyre::eyre!("Unable to find config directory for tuisic"));
        };
        Ok(directory)
    }
}
