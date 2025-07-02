use std::env;

use super::{ConfigFile, LocalConfigFile, ProviderConfigFile};

pub struct Defaults;

impl Defaults {
    pub fn config() -> ConfigFile {
        ConfigFile {
            providers: Self::providers(),
        }
    }

    pub fn providers() -> Vec<ProviderConfigFile> {
        vec![ProviderConfigFile::Local {
            config: Self::local_config(),
        }]
    }

    pub fn local_config() -> LocalConfigFile {
        let home_path = env::home_dir().unwrap();

        LocalConfigFile {
            path: format!("{}/Music", home_path.display()),
        }
    }
}
