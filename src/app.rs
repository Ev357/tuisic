use crate::{
    app_config::{AppConfig, providers::ProviderConfig},
    providers::{LocalProvider, Provider},
    song::Song,
    view::{
        View,
        library::{LibraryView, items::Items},
    },
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
    view: View,
}

pub enum AppEvent {
    Quit,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = AppConfig::new()?;

        let providers = Self::create_providers(config);

        let songs = Self::load_all_songs(&providers)?;

        let view = View::Library {
            view: LibraryView {
                items: Items::from_iter(songs),
            },
        };

        Ok(Self { view })
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

    pub fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        let mut should_exit = false;

        while !should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;

            if let Event::Key(key) = event::read()?
                && let Some(_) = self.handle_key(key)
            {
                should_exit = true;
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<AppEvent> {
        if key.kind != KeyEventKind::Press {
            return None;
        }

        match self.view {
            View::Library { ref mut view } => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Some(AppEvent::Quit),
                KeyCode::Char('j') | KeyCode::Down => view.items.state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => view.items.state.select_previous(),
                KeyCode::Char('g') | KeyCode::Home => view.items.state.select_first(),
                KeyCode::Char('G') | KeyCode::End => view.items.state.select_last(),
                KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                    return Some(AppEvent::Quit);
                }
                _ => {}
            },
            View::Song => {}
            View::Search => {}
        }

        None
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        match self.view {
            View::Library { ref mut view } => {
                let items: Vec<ListItem> = view
                    .items
                    .items
                    .iter()
                    .map(|item| ListItem::new(Line::from(format!(" {}", item.title))))
                    .collect();

                let list = List::new(items)
                    .highlight_symbol(">")
                    .highlight_spacing(HighlightSpacing::Always);

                StatefulWidget::render(list, area, buf, &mut view.items.state);
            }
            View::Song => {}
            View::Search => {}
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_list(area, buf);
    }
}
