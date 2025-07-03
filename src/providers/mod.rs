use crate::song::Song;
use color_eyre::Result;
pub use local::LocalProvider;

pub mod local;

pub trait Provider {
    fn get_songs(&self) -> Result<Vec<Song>>;
}
