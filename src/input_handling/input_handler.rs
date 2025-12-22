use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::error;

use crate::{app::App, input_handling::directory, state::diode::selected_entry::SelectedEntry};

pub fn on_key_event(app: &mut App, key: KeyEvent) {
    let (current_state, other_state) = app.diode_state.get_states_mut();
    match (key.modifiers, key.code) {
        (_, KeyCode::Esc | KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
        (_, KeyCode::Tab) => app.diode_state.switch_selection(),
        (_, KeyCode::Char('j')) => current_state.move_down(),
        (_, KeyCode::Char('k')) => current_state.move_up(),
        (_, KeyCode::Backspace) => current_state.set_parent_as_new_root(),
        (_, KeyCode::Char('m')) => {
            let Some(selected) = &current_state.selected else {
                return;
            };

            let entries = match other_state.move_marked(selected) {
                Ok(entries) => entries,
                Err(e) => {
                    error!("{}", e);
                    return;
                }
            };

            current_state.reload(entries)
        }
        (_, KeyCode::Char(' ')) => {
            let Some(selected_entry) = current_state.get_selected_entry_mut() else {
                return;
            };

            selected_entry.toggle_marked();
        }
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
