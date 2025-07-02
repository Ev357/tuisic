use super::{ConfigFile, LocalConfig, ProviderConfigFile};
use std::sync::Arc;

#[derive(Clone)]
pub struct Config {
    pub providers: Arc<Vec<ProviderConfig>>,
}

#[derive(Clone)]
pub enum ProviderConfig {
    Local { config: Arc<LocalConfig> },
}

impl From<ConfigFile> for Config {
    fn from(file_config: ConfigFile) -> Self {
        Config {
            providers: Arc::new(
                file_config
                    .providers
                    .into_iter()
                    .map(|p| p.into())
                    .collect(),
            ),
        }
    }
}

impl From<ProviderConfigFile> for ProviderConfig {
    fn from(file_provider: ProviderConfigFile) -> Self {
        match file_provider {
            ProviderConfigFile::Local { config } => ProviderConfig::Local {
                config: Arc::new(config.into()),
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        ConfigFile::default().into()
    }
}
