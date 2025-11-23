// TODO: Remove
#![allow(dead_code)]
mod app;
mod file_management;
mod ui;

use crate::file_management::entry::EntryType;
use app::App;
use std::fs;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let result = fs::read_dir("./")?;
    let dir = EntryType::try_from(&result.into_iter().next().unwrap()?)?;
    println!("{:#?}", dir);
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
