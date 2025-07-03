use crate::app_config::providers::local::LocalConfig;
use serde::Deserialize;

pub mod local;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ProviderConfig {
    #[serde(rename = "local")]
    Local {
        #[serde(flatten)]
        config: LocalConfig,
    },
}
