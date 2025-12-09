use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use log::error;

pub fn on_key_event(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Esc | KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
        (_, KeyCode::Tab) => app.diode_state.switch_selection(),
        (_, KeyCode::Char('j')) => app.diode_state.get_current_state_mut().move_down(),
        (_, KeyCode::Char('k')) => app.diode_state.get_current_state_mut().move_up(),
        // TODO: This should only be available if the selected entry is a dir
        (_, KeyCode::Enter) => {
            if let Err(e) = app.diode_state.get_current_state_mut().toggle_dir() {
                error!("{:?}", e);
            }
        }
        (_, KeyCode::Char('r')) => app.diode_state.get_current_state_mut().set_dir_as_root(),
        (_, KeyCode::Backspace) => app
            .diode_state
            .get_current_state_mut()
            .set_parent_as_new_root(),
        _ => {}
    }
}
