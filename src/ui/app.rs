use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::ui::body;
use crate::ui::footer;
use crate::ui::header;

pub fn draw(frame: &mut Frame) {
    let chunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(0),
        Constraint::Length(2),
    ])
    .split(frame.area());

    frame.render_widget(header::new("duality"), chunks[0]);

    let block = body::create_block();
    let inner = block.inner(chunks[1]);
    let [layout_left, layout_center, layout_right] = body::create_layout(inner);
    frame.render_widget(body::create_pane("left"), layout_left);
    frame.render_widget(body::separator(), layout_center);
    frame.render_widget(body::create_pane("right"), layout_right);

    frame.render_widget(footer::new(), chunks[2]);
}
