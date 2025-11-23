use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::List,
};

use crate::{file_management::entry::EntryType, state::file_state::FileState};

pub struct LayoutPanePair {
    pub rect: Rect,
    pub pane: List<'static>,
}

pub fn new(area: Rect, file_state: &FileState) -> [LayoutPanePair; 2] {
    let rects = create_layout(area);
    [
        LayoutPanePair {
            rect: rects[0],
            pane: create_pane(file_state.left_dir.entries()),
        },
        LayoutPanePair {
            rect: rects[1],
            pane: create_pane(file_state.right_dir.entries()),
        },
    ]
}

fn create_layout(area: Rect) -> [Rect; 2] {
    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    [rects[0], rects[1]]
}

fn create_pane(entries: &[EntryType]) -> List<'static> {
    todo!()
}
