use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    text::Line,
    widgets::{HighlightSpacing, List, ListItem, StatefulWidget, Widget},
};

use crate::file_list::FileList;

pub struct App {
    file_list: FileList,
}

pub enum AppEvent {
    Quit,
    Select(String),
}

impl App {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            file_list: FileList::from_iter(files),
        }
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
            KeyCode::Char('j') | KeyCode::Down => self.file_list.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.file_list.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.file_list.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.file_list.select_last(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                let selected_item = self.file_list.selected_item().unwrap();

                return Some(AppEvent::Select(selected_item));
            }
            _ => {}
        }

        None
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .file_list
            .items
            .iter()
            .map(|item| ListItem::new(Line::from(format!(" {item}"))))
            .collect();

        let list = List::new(items)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.file_list.state);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_list(area, buf);
    }
}
