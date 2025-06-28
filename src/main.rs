use color_eyre::Result;

mod app;
mod file_item;
mod file_list;
mod file_utils;

use app::App;
use file_utils::get_files;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let files = get_files()?;
    let app_result = App::new(files).run(terminal)?;

    ratatui::restore();

    if let Some(selected) = app_result {
        println!("Selected: {selected}");
    }

    Ok(())
}
