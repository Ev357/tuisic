use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct LocalConfig {
    pub path: PathBuf,
}
