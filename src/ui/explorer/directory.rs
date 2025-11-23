use ratatui::widgets::ListItem;

use crate::file_management::directory::Directory;

pub fn create_list_item(directory: &Directory) -> ListItem<'_> {
    ListItem::new(format!("📁 {}", directory.name().to_string_lossy()))
}
