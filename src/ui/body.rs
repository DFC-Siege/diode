use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};

pub struct LayoutPanePair {
    pub rect: Rect,
    pub pane: Paragraph<'static>,
}

pub fn new(area: Rect) -> [LayoutPanePair; 2] {
    let rects = create_layout(area);
    [
        LayoutPanePair {
            rect: rects[0],
            pane: create_pane("Left Pane"),
        },
        LayoutPanePair {
            rect: rects[1],
            pane: create_pane("Right Pane"),
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

fn create_pane(text: &str) -> Paragraph<'static> {
    Paragraph::new(text.to_string())
        .block(Block::default().borders(Borders::ALL))
        .centered()
}
