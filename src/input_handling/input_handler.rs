use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn on_key_event(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Esc | KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
        (_, KeyCode::Tab) => app.diode_state.switch_selection(),
        (_, KeyCode::Char('j')) => app.diode_state.move_down(),
        (_, KeyCode::Char('k')) => app.diode_state.move_up(),
        // TODO: This should only be available if the selected entry is a dir
        (_, KeyCode::Enter) => app.diode_state.toggle_dir(),
        _ => {}
    }
}
