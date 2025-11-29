use crate::state::diode::file_state::FileState;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::ListItem,
};

pub fn create_list_item(file: &FileState, indent: u8) -> ListItem<'_> {
    let tabs = "  ".repeat(indent as usize);
    let mut item = ListItem::new(format!("{}📄 {}", tabs, file.name.to_string_lossy()));

    if file.selected.get() {
        item = item.style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    }

    item
}
