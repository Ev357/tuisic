use core::fmt;
use ratatui::{text::Line, widgets::ListItem};

#[derive(Debug, Clone)]
pub struct FileItem(pub String);

impl FileItem {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl fmt::Display for FileItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&FileItem> for ListItem<'_> {
    fn from(value: &FileItem) -> Self {
        ListItem::new(Line::from(format!(" {value}")))
    }
}
