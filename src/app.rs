use std::sync::{Arc, RwLock};

use color_eyre::Result;

use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    text::Line,
    widgets::{HighlightSpacing, List, ListItem, StatefulWidget, Widget},
};

use crate::{
    config::{Config, ProviderConfig},
    providers::{LocalProvider, Provider},
    song::Song,
    song_list::SongList,
};

pub struct App {
    song_list: SongList,
}

pub enum AppEvent {
    Quit,
    Select { song: Song },
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Self::load_config()?;

        let providers = Self::create_providers(Arc::clone(&config));

        let songs = Self::load_all_songs(&providers)?;

        let song_list = SongList::from_iter(songs);

        Ok(Self { song_list })
    }

    fn load_config() -> Result<Arc<RwLock<Config>>> {
        let config = Config::from_file("~/.config/tuisic/config.toml")?;

        Ok(Arc::new(RwLock::new(config)))
    }

    fn load_all_songs(providers: &[Box<dyn Provider>]) -> Result<Vec<Song>> {
        providers
            .iter()
            .map(|provider| provider.get_songs())
            .collect::<Result<Vec<Vec<Song>>>>()
            .map(|song_vecs| song_vecs.into_iter().flatten().collect())
    }

    fn create_providers(config: Arc<RwLock<Config>>) -> Vec<Box<dyn Provider>> {
        config
            .read()
            .unwrap()
            .providers
            .iter()
            .map(|provider| match provider {
                ProviderConfig::Local { config } => {
                    Box::new(LocalProvider::new(Arc::clone(config))) as Box<dyn Provider>
                }
            })
            .collect()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<AppEvent> {
        let mut should_exit = false;

        while !should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            if let Event::Key(key) = event::read()?
                && let Some(event) = self.handle_key(key)
            {
                if let AppEvent::Quit = event {
                    should_exit = true;
                } else {
                    return Ok(event);
                }
            }
        }

        Ok(AppEvent::Quit)
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<AppEvent> {
        if key.kind != KeyEventKind::Press {
            return None;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Some(AppEvent::Quit),
            KeyCode::Char('j') | KeyCode::Down => self.song_list.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.song_list.state.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.song_list.state.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.song_list.state.select_last(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                let song = self.song_list.selected_item().unwrap();

                return Some(AppEvent::Select { song });
            }
            _ => {}
        }

        None
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .song_list
            .songs
            .iter()
            .map(|item| ListItem::new(Line::from(format!(" {}", item.title))))
            .collect();

        let list = List::new(items)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.song_list.state);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_list(area, buf);
    }
}
