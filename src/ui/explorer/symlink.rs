use ratatui::widgets::ListItem;

use crate::file_management::symlink::Symlink;

pub fn create_list_item(symlink: &Symlink) -> ListItem<'_> {
    ListItem::new(format!("🔗 {}", symlink.name().to_string_lossy()))
}
