use crate::file_item::FileItem;
use ratatui::widgets::ListState;

pub struct FileList {
    pub items: Vec<FileItem>,
    pub state: ListState,
}

impl FromIterator<String> for FileList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let items = iter.into_iter().map(|s| FileItem::new(&s)).collect();
        let mut state = ListState::default();
        state.select_first();
        Self { items, state }
    }
}

impl FileList {
    pub fn selected_item(&self) -> Option<&FileItem> {
        self.state.selected().map(|i| &self.items[i])
    }

    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    }
}
