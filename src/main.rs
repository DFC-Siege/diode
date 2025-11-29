// TODO: Remove
#![allow(dead_code)]
mod app;
mod file_management;
mod input_handling;
mod state;
mod ui;

use crate::{
    file_management::directory::Directory,
    state::diode::{
        diode_state::DiodeState, directory_state::DirectoryState, explorer_state::ExplorerState,
    },
};
use app::App;
use std::env;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let current_dir = env::current_dir()?;
    let left_explorer = ExplorerState::try_new(DirectoryState::from(Directory::try_from(
        current_dir.clone(),
    )?))?;
    println!("{:#?}", left_explorer);
    let right_explorer = ExplorerState::try_new(DirectoryState::from(Directory::try_from(
        current_dir.clone(),
    )?))?;
    let terminal = ratatui::init();
    let diode_state = DiodeState::new(left_explorer, right_explorer);
    let result = App::new(diode_state).run(terminal).await;
    ratatui::restore();
    result
}
