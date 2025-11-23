use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::ui::body;
use crate::ui::header;
use crate::{state::file_system_state::FileSystemState, ui::footer};

pub fn draw(frame: &mut Frame, file_system_state: &FileSystemState) {
    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(frame.area());

    frame.render_widget(header::new("diode"), chunks[0]);

    let [layout_left, layout_right] = body::new(chunks[1], file_system_state);
    frame.render_widget(layout_left.pane, layout_left.rect);
    frame.render_widget(layout_right.pane, layout_right.rect);

    frame.render_widget(footer::new(), chunks[2]);
}
