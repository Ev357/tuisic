use crate::view::library::LibraryView;

pub mod library;

#[allow(dead_code)]
pub enum View {
    Library { view: LibraryView },
    Song,
    Search,
}
