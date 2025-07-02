use super::defaults::Defaults;
use super::local::LocalConfigFile;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    #[serde(default = "Defaults::providers")]
    pub providers: Vec<ProviderConfigFile>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ProviderConfigFile {
    #[serde(rename = "local")]
    Local {
        #[serde(flatten)]
        config: LocalConfigFile,
    },
}

impl Default for ConfigFile {
    fn default() -> Self {
        Defaults::config()
    }
}
