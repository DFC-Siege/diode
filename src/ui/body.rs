use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::{
    state::diode::diode_state::{DiodeState, Selection},
    ui::explorer::explorer_pane::{self, ExplorerPane},
};

pub struct LayoutPanePair<'a> {
    pub rect: Rect,
    pub pane: ExplorerPane<'a>,
}

pub fn new(area: Rect, diode_state: &DiodeState) -> [LayoutPanePair<'_>; 2] {
    let rects = create_layout(area);
    [
        LayoutPanePair {
            rect: rects[0],
            pane: explorer_pane::create_pane(
                &diode_state.left_state.entries,
                diode_state.selected == Selection::Left,
            ),
        },
        LayoutPanePair {
            rect: rects[1],
            pane: explorer_pane::create_pane(
                &diode_state.right_state.entries,
                diode_state.selected == Selection::Right,
            ),
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
