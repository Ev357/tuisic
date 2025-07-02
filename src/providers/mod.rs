pub mod local;
use color_eyre::Result;

pub use local::LocalProvider;

use crate::song::Song;

pub trait Provider {
    fn get_songs(&self) -> Result<Vec<Song>>;
}
