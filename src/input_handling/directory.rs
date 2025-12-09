use crate::state::diode::explorer_state::SelectedDirectory;
use crossterm::event::{KeyCode, KeyEvent};
use log::error;

pub fn on_key_event(key: KeyEvent, mut selected: SelectedDirectory) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Enter) => {
            if let Err(e) = selected.toggle_dir() {
                error!("{:?}", e);
            }
        }
        (_, KeyCode::Char('r')) => selected.set_dir_as_root(),
        (_, KeyCode::Backspace) => selected.set_parent_as_new_root(),
        _ => {}
    }
}
