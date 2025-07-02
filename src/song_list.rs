use crate::song::Song;
use ratatui::widgets::ListState;

pub struct SongList {
    pub songs: Vec<Song>,
    pub state: ListState,
}

impl FromIterator<Song> for SongList {
    fn from_iter<I: IntoIterator<Item = Song>>(iter: I) -> Self {
        let songs = iter.into_iter().collect();

        let mut state = ListState::default();
        state.select_first();

        Self { songs, state }
    }
}

impl SongList {
    pub fn selected_item(&self) -> Option<Song> {
        self.state.selected().map(|i| self.songs[i].clone())
    }
}
