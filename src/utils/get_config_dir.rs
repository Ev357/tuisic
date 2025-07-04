use crate::utils::project_directory::project_directory;
use color_eyre::eyre::{self, Result};
use std::path::PathBuf;

pub fn get_config_dir() -> Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("TUISIC_CONFIG") {
        PathBuf::from(s)
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        return Err(eyre::eyre!("Unable to find config directory for tuisic"));
    };
    Ok(directory)
}
