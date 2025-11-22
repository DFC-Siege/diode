use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::ui::footer;
use crate::ui::header;

pub fn draw(frame: &mut Frame) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ])
    .split(frame.area());

    frame.render_widget(header::new("duality"), chunks[0]);
    frame.render_widget(footer::new("footer"), chunks[2]);
}
