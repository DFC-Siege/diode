use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, input_handling::directory, state::diode::selected_entry::SelectedEntry};

pub fn on_key_event(app: &mut App, key: KeyEvent) {
    let current_state = app.diode_state.get_current_state_mut();
    match (key.modifiers, key.code) {
        (_, KeyCode::Esc | KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
        (_, KeyCode::Tab) => app.diode_state.switch_selection(),
        (_, KeyCode::Char('j')) => current_state.move_down(),
        (_, KeyCode::Char('k')) => current_state.move_up(),
        _ => {
            let Some(selected_state) = current_state.with_selected() else {
                return;
            };
            match selected_state {
                SelectedEntry::Directory(v) => directory::on_key_event(key, v),
            };
        }
    }
}
