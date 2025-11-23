use crate::state::diode::directory_state::DirectoryState;

#[derive(Debug)]
pub struct DiodeState {
    pub left_state: DirectoryState,
    pub right_state: DirectoryState,
}

impl DiodeState {
    pub fn new(left_state: DirectoryState, right_state: DirectoryState) -> Self {
        Self {
            left_state,
            right_state,
        }
    }
}
