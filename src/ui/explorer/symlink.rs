use ratatui::widgets::ListItem;

use crate::state::diode::symlink_state::SymlinkState;

pub fn create_list_item(symlink: &SymlinkState) -> ListItem<'_> {
    ListItem::new(format!("🔗 {}", symlink.name.to_string_lossy()))
}
