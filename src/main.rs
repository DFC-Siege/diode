// TODO: Remove
#![allow(dead_code)]
mod app;
mod file_management;
mod state;
mod ui;

use crate::{
    file_management::{directory::Directory, entry::EntryType},
    state::file_state::FileState,
};
use app::App;
use std::{env, fs};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let current_dir = env::current_dir()?;
    let left_dir = Directory::try_from(&current_dir)?;
    let right_dir = Directory::try_from(&current_dir)?;
    let terminal = ratatui::init();
    let file_state = FileState::new(left_dir, right_dir);
    let result = App::new(file_state).run(terminal).await;
    ratatui::restore();
    result
}
