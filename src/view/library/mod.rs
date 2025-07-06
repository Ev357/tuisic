use crate::{song::Song, view::library::items::Items};

pub mod items;

pub struct LibraryView {
    pub items: Items<Song>,
}
