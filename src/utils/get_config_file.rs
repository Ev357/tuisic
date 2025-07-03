use std::path::PathBuf;

use color_eyre::eyre::{self, Result};
use directories::ProjectDirs;

pub fn get_config_file() -> Result<PathBuf> {
    let directory = get_config_dir()?;

    Ok(directory.join("config.toml"))
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
