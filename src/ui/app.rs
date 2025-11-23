use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::ui::footer;
use crate::ui::header;
use crate::{state::diode::diode_state::DiodeState, ui::body};

pub fn draw(frame: &mut Frame, diode_state: &DiodeState) {
    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(frame.area());

    frame.render_widget(header::new("diode"), chunks[0]);

    let [layout_left, layout_right] = body::new(chunks[1], diode_state);
    frame.render_widget(layout_left.pane, layout_left.rect);
    frame.render_widget(layout_right.pane, layout_right.rect);

    frame.render_widget(footer::new(), chunks[2]);
}
