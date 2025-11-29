use ratatui::widgets::ListItem;

use crate::state::diode::symlink_state::SymlinkState;

pub fn create_list_item(symlink: &SymlinkState, indent: u8) -> ListItem<'_> {
    let tabs = "  ".repeat(indent as usize);
    ListItem::new(format!("{}🔗 {}", tabs, symlink.name.to_string_lossy()))
}
