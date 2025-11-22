use ratatui::widgets::{Block, Paragraph};

pub fn new() -> Paragraph<'static> {
    Paragraph::new("footer").block(Block::default()).centered()
}
