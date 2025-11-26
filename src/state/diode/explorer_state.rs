use std::rc::Weak;

use crate::state::diode::{directory_state::DirectoryState, entry_state::EntryState};

struct ExplorerState {
    pub root: DirectoryState,
    pub selected_entry: Weak<EntryState>,
}
