use color_eyre::Result;
use std::fs;

pub fn get_files() -> Result<Vec<String>> {
    let entries = fs::read_dir(".")?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();

    Ok(entries)
}
