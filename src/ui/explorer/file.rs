use crate::state::diode::file_state::FileState;
use ratatui::widgets::ListItem;

pub fn create_list_item(file: &FileState, indent: u8) -> ListItem<'_> {
    let tabs = "  ".repeat(indent as usize);
    ListItem::new(format!("{}📄 {}", tabs, file.name.to_string_lossy()))
}
