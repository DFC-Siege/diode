use ratatui::widgets::{Block, Paragraph};

pub fn new(title: &str) -> Paragraph<'_> {
    Paragraph::new(title).block(Block::default()).centered()
}
