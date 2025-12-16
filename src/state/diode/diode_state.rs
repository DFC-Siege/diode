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

    pub fn get_states_mut(&mut self) -> (&mut ExplorerState, &mut ExplorerState) {
        match self.selected {
            Selection::Left => (&mut self.left_state, &mut self.right_state),
            Selection::Right => (&mut self.right_state, &mut self.left_state),
        }
    }
}
