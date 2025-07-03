use crate::{app_config::providers::local::LocalConfig, providers::Provider, song::Song};
use color_eyre::Result;
use std::fs;

pub struct LocalProvider {
    config: LocalConfig,
}

impl LocalProvider {
    pub fn new(config: LocalConfig) -> Self {
        Self { config }
    }
}

impl Provider for LocalProvider {
    fn get_songs(&self) -> Result<Vec<Song>> {
        let songs_path_config = &self.config.path.to_string_lossy().to_string();

        let songs_path = shellexpand::full(songs_path_config).unwrap();

        Ok(fs::read_dir(songs_path.as_ref())?
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|entry| {
                let title = entry.file_name().to_string_lossy().to_string();
                Song { title }
            })
            .collect())
    }
}
