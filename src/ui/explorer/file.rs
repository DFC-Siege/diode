use ratatui::widgets::ListItem;

use crate::state::diode::file_state::FileState;

pub fn create_list_item(file: &FileState) -> ListItem<'_> {
    ListItem::new(format!("📄 {}", file.name.to_string_lossy()))
}
