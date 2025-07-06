use app::App;
use color_eyre::Result;
use ratatui::{Terminal, prelude::Backend};

mod app;
mod app_config;
mod logging;
mod providers;
mod song;
mod tui;
mod utils;
mod view;

fn main() -> Result<()> {
    color_eyre::install()?;
    logging::initialize_logging()?;
    let terminal = tui::init()?;

    let result = run(terminal);

    tui::restore()?;

    result
}

fn run(terminal: Terminal<impl Backend>) -> Result<()> {
    let app = App::new()?;

    app.run(terminal)
}
