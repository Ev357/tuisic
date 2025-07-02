use super::defaults::Defaults;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LocalConfigFile {
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct LocalConfig {
    pub path: String,
}

impl From<LocalConfigFile> for LocalConfig {
    fn from(file_config: LocalConfigFile) -> Self {
        LocalConfig {
            path: file_config.path,
        }
    }
}

impl Default for LocalConfigFile {
    fn default() -> Self {
        Defaults::local_config()
    }
}

impl Default for LocalConfig {
    fn default() -> Self {
        LocalConfigFile::default().into()
    }
}
