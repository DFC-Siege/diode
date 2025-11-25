use crate::state::diode::directory_state::DirectoryState;

#[derive(Debug)]
pub struct DiodeState {
    pub left_state: DirectoryState,
    pub right_state: DirectoryState,
    pub selected: Selection,
}

#[derive(Debug, PartialEq)]
pub enum Selection {
    Left,
    Right,
}

impl DiodeState {
    pub fn new(left_state: DirectoryState, right_state: DirectoryState) -> Self {
        Self {
            left_state,
            right_state,
            selected: Selection::Left,
        }
    }

    pub fn switch_selection(&mut self) {
        self.selected = match self.selected {
            Selection::Left => Selection::Right,
            Selection::Right => Selection::Left,
        };
    }
}
