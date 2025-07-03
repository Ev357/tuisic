use std::{fs, path::Path};

use color_eyre::Result;
pub use file_format::{ConfigFile, ProviderConfigFile};
pub use local::{LocalConfig, LocalConfigFile};
pub use runtime::{Config, ProviderConfig};

mod defaults;
mod file_format;
pub mod local;
mod runtime;

impl Config {
    pub fn from_file(path: &Path) -> Result<Config> {
        if let Ok(content) = fs::read_to_string(path) {
            let file_config: ConfigFile = toml::from_str(&content)?;
            return Ok(file_config.into());
        }
        Ok(Config::default())
    }
}
