use ratatui::widgets::{Block, Borders, Paragraph};

pub fn new(title: &str) -> Paragraph<'_> {
    Paragraph::new(title)
        .block(Block::default().borders(Borders::BOTTOM))
        .centered()
}
