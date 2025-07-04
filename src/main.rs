use crate::app::AppEvent;
use app::App;
use color_eyre::Result;
use ratatui::{Terminal, prelude::Backend};

mod app;
mod app_config;
mod logging;
mod providers;
mod song;
mod song_list;
mod tui;
mod utils;

fn main() -> Result<()> {
    color_eyre::install()?;
    logging::initialize_logging()?;
    let terminal = tui::init()?;

    let result = run(terminal);

    tui::restore()?;

    match result {
        Ok(AppEvent::Select { song }) => {
            println!("Selected: {}", song.title);
        }
        Ok(AppEvent::Quit) => {
            println!("No song selected");
        }
        Err(error) => {
            return Err(error);
        }
    }

    Ok(())
}

fn run(terminal: Terminal<impl Backend>) -> Result<AppEvent> {
    let app = App::new()?;

    app.run(terminal)
}
