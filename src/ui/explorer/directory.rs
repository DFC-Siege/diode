use ratatui::widgets::ListItem;

use crate::state::diode::directory_state::DirectoryState;

pub fn create_list_item(directory: &DirectoryState) -> ListItem<'_> {
    match directory.collapsed {
        true => ListItem::new(format!("📁 {}", directory.name.to_string_lossy())),
        false => ListItem::new(format!("📂 {}", directory.name.to_string_lossy())),
    }
}
