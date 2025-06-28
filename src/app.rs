use color_eyre::Result;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    widgets::{HighlightSpacing, List, ListItem, StatefulWidget, Widget},
};

use crate::file_list::FileList;

pub struct App {
    should_exit: bool,
    file_list: FileList,
}

impl App {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            should_exit: false,
            file_list: FileList::from_iter(files),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<Option<String>> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }

        Ok(self.file_list.selected_item().map(|item| item.0.clone()))
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.file_list.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.file_list.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.file_list.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.file_list.select_last(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => self.should_exit = true,
            _ => {}
        }
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self.file_list.items.iter().map(ListItem::from).collect();

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
