use color_eyre::Result;
use core::fmt;
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    text::Line,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal)?;
    ratatui::restore();

    println!("Selected: {}", app_result);
    Ok(())
}

struct App {
    should_exit: bool,
    todo_list: TodoList,
}

struct TodoList {
    items: Vec<TodoItem>,
    state: ListState,
}

struct TodoItem(String);

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            todo_list: TodoList::from_iter([
                "Rewrite everything with Rust!",
                "Rewrite all of your tui apps with Ratatui",
                "Pet your cat",
                "Walk with your dog",
                "Pay the bills",
                "Refactor list example",
            ]),
        }
    }
}

impl FromIterator<&'static str> for TodoList {
    fn from_iter<I: IntoIterator<Item = &'static str>>(iter: I) -> Self {
        let items = iter.into_iter().map(|todo| TodoItem::new(todo)).collect();
        let mut state = ListState::default();
        state.select_first();
        Self { items, state }
    }
}

impl TodoItem {
    fn new(todo: &str) -> Self {
        Self(todo.to_string())
    }
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<String> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        if let Some(selected) = self.todo_list.state.selected() {
            Ok(self.todo_list.items[selected].0.clone())
        } else {
            Ok(String::new())
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => self.should_exit = true,
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.todo_list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.todo_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.todo_list.state.select_last();
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_list(area, buf);
    }
}

impl App {
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new();

        let items: Vec<ListItem> = self
            .todo_list
            .items
            .iter()
            .map(|todo_item| ListItem::from(todo_item))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
    }
}

impl From<&TodoItem> for ListItem<'_> {
    fn from(value: &TodoItem) -> Self {
        ListItem::new(Line::from(format!(" {}", value)))
    }
}
