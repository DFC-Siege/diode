use ratatui::widgets::{List, ListItem};

use crate::file_management::entry::EntryType;

pub fn create_list(entries: &[EntryType]) -> List<'_> {
    let items: Vec<ListItem> = entries
        .iter()
        .map(|v| ListItem::new(v.name().to_string_lossy()))
        .collect();

    List::new(items)
}
