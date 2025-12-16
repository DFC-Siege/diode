use crossterm::event::{KeyCode, KeyEvent};
use log::error;

use crate::state::diode::focussed::directory::FocussedDirectory;

pub fn on_key_event(key: KeyEvent, mut focussed: FocussedDirectory) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Enter) => {
            if let Err(e) = focussed.toggle_dir() {
                error!("{:?}", e);
            }
        }
        (_, KeyCode::Char('r')) => focussed.set_dir_as_root(),
        _ => {}
    }
}
