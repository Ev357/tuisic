use crate::song::Song;
use ratatui::widgets::ListState;

pub struct Items<T> {
    pub items: Vec<T>,
    pub state: ListState,
}

impl FromIterator<Song> for Items<Song> {
    fn from_iter<I: IntoIterator<Item = Song>>(iter: I) -> Self {
        let items = iter.into_iter().collect();

        let mut state = ListState::default();
        state.select_first();

        Self { items, state }
    }
}
