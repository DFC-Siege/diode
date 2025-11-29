use crate::state::diode::symlink_state::SymlinkState;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::ListItem,
};

pub fn create_list_item(symlink: &SymlinkState, indent: u8) -> ListItem<'_> {
    let tabs = "  ".repeat(indent as usize);
    let mut item = ListItem::new(format!("{}🔗 {}", tabs, symlink.name.to_string_lossy()));

    if symlink.selected.get() {
        item = item.style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    }

    item
}
