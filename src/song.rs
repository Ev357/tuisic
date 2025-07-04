use std::{borrow::Cow, path::Path};

use color_eyre::eyre::Result;
use lofty::{
    file::TaggedFileExt,
    probe::Probe,
    tag::{Accessor, Tag},
};

#[derive(Clone)]
pub struct Song {
    pub title: String,
}

impl Song {
    pub fn new(path: &Path) -> Result<Self> {
        let tagged_file = Probe::open(path)?.read()?;
        let tag = tagged_file.primary_tag();

        let title = Self::get_title(tag, path).into_owned();

        Ok(Self { title })
    }

    fn get_title<'a>(tag: Option<&'a Tag>, path: &'a Path) -> Cow<'a, str> {
        if let Some(tag) = tag
            && let Some(title) = tag.title()
        {
            return title;
        }

        path.file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("Unknown Title")
            .into()
    }
}
