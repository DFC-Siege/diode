use ratatui::widgets::{Block, Borders, Paragraph};

pub fn new() -> Paragraph<'static> {
    Paragraph::new("footer")
        .block(Block::default().borders(Borders::TOP))
        .centered()
}
