use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};

pub fn create_block() -> Block<'static> {
    Block::default().title("Content")
}

pub fn create_layout(area: Rect) -> [Rect; 3] {
    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Percentage(50),
        ])
        .split(area);

    [rects[0], rects[1], rects[2]]
}

pub fn create_pane(text: &str) -> Paragraph<'_> {
    Paragraph::new(text).block(Block::default()).centered()
}

pub fn separator() -> Block<'static> {
    Block::default().borders(Borders::LEFT)
}
