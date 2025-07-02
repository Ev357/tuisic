use color_eyre::Result;
use std::{fs, sync::Arc};

use crate::{config::LocalConfig, providers::Provider, song::Song};

#[derive(Debug)]
pub struct LocalProvider {
    config: Arc<LocalConfig>,
}

impl LocalProvider {
    pub fn new(config: Arc<LocalConfig>) -> Self {
        Self { config }
    }
}

impl Provider for LocalProvider {
    fn get_songs(&self) -> Result<Vec<Song>> {
        Ok(fs::read_dir(&self.config.path)?
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|entry| {
                let title = entry.file_name().to_string_lossy().to_string();
                Song { title }
            })
            .collect())
    }
}
