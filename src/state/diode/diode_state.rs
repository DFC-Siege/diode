use crate::state::diode::explorer_state::ExplorerState;

#[derive(Debug)]
pub struct DiodeState {
    pub left_state: ExplorerState,
    pub right_state: ExplorerState,
    pub selected: Selection,
}

#[derive(Debug, PartialEq)]
pub enum Selection {
    Left,
    Right,
}

impl DiodeState {
    pub fn new(left_state: ExplorerState, right_state: ExplorerState) -> Self {
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

    pub fn move_down(&mut self) {
        match self.selected {
            Selection::Left => self.left_state.move_down(),
            Selection::Right => self.right_state.move_down(),
        };
    }

    pub fn move_up(&mut self) {
        match self.selected {
            Selection::Left => self.left_state.move_up(),
            Selection::Right => self.right_state.move_up(),
        };
    }

    pub fn toggle_dir(&mut self) {
        match self.selected {
            Selection::Left => self.left_state.toggle_dir(),
            Selection::Right => self.right_state.toggle_dir(),
        }
        .ok();
    }
}
