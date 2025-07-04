use crate::utils::get_config_dir::get_config_dir;
use color_eyre::eyre::Result;
use std::path::PathBuf;

pub fn get_config_file() -> Result<Option<PathBuf>> {
    let directory = get_config_dir()?;

    let config_file = directory.join("config.toml");

    if !config_file.exists() {
        return Ok(None);
    }

    Ok(Some(config_file))
}
