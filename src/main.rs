mod app;
mod file_list;
mod file_utils;

use app::App;

use crate::app::AppEvent;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let files = file_utils::get_files()?;
    let app_result = App::new(files).run(terminal)?;

    ratatui::restore();

    if let AppEvent::Select(selected) = app_result {
        println!("Selected: {selected}");
    } else {
        println!("No file selected");
    }

    Ok(())
}
