use ratatui::widgets::ListItem;

use crate::file_management::file::File;

pub fn create_list_item(file: &File) -> ListItem<'_> {
    ListItem::new(format!("📄 {}", file.name().to_string_lossy()))
}
