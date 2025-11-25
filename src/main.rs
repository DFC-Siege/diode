// TODO: Remove
#![allow(dead_code)]
mod app;
mod file_management;
mod input_handling;
mod state;
mod ui;

use crate::{file_management::directory::Directory, state::diode::diode_state::DiodeState};
use app::App;
use std::env;

fn test_result() -> Result<String, String> {
    if true {
        Ok("success".to_owned())
    } else {
        Err("If statement failed".to_owned())
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let current_dir = env::current_dir()?;
    let left_dir = Directory::try_from(&current_dir)?;
    let right_dir = Directory::try_from(&current_dir)?;
    let terminal = ratatui::init();
    let diode_state = DiodeState::new(left_dir.into(), right_dir.into());
    let result = App::new(diode_state).run(terminal).await;
    ratatui::restore();
    result
}
