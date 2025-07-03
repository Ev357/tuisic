use crate::{
    app_config::{AppConfig, providers::ProviderConfig},
    providers::{LocalProvider, Provider},
    song::Song,
    song_list::SongList,
};
use color_eyre::Result;
use ratatui::{
    Terminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    prelude::Backend,
    text::Line,
    widgets::{HighlightSpacing, List, ListItem, StatefulWidget, Widget},
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
        let config = AppConfig::new()?;

        let providers = Self::create_providers(config);

        let songs = Self::load_all_songs(&providers)?;

        let song_list = SongList::from_iter(songs);

        Ok(Self { song_list })
    }

    fn load_all_songs(providers: &[Box<dyn Provider>]) -> Result<Vec<Song>> {
        providers
            .iter()
            .map(|provider| provider.get_songs())
            .collect::<Result<Vec<Vec<Song>>>>()
            .map(|song_vecs| song_vecs.into_iter().flatten().collect())
    }

    fn create_providers(config: AppConfig) -> Vec<Box<dyn Provider>> {
        config
            .providers
            .into_iter()
            .map(|provider| -> Box<dyn Provider> {
                match provider {
                    ProviderConfig::Local { config } => Box::new(LocalProvider::new(config)),
                }
            })
            .collect()
    }

    pub fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<AppEvent> {
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
