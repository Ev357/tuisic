mod app;
mod config;
mod providers;
mod song;
mod song_list;

use color_eyre::Result;

use app::{App, AppEvent};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let app_result = App::new()?.run(terminal)?;

    ratatui::restore();

    if let AppEvent::Select { song } = app_result {
        println!("Selected: {}", song.title);
    } else {
        println!("No song selected");
    }

    Ok(())
}
