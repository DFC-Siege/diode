use std::{iter::once, rc::Rc};

use ratatui::widgets::ListItem;

use crate::{
    state::diode::{directory_state::DirectoryState, entry_state::EntryState},
    ui::explorer::{file, symlink},
};

pub fn create_list_item(directory: &DirectoryState, indent: u8) -> Vec<ListItem<'_>> {
    let mut items: Vec<ListItem> = Vec::new();
    let tabs = "  ".repeat(indent as usize);
    items.push(match directory.collapsed {
        true => ListItem::new(format!("{}📁 {}", tabs, directory.name.to_string_lossy())),
        false => ListItem::new(format!("{}📂 {}", tabs, directory.name.to_string_lossy())),
    });

    items.append(&mut create_list(&directory.entries, indent + 1));

    items
}

fn create_list(entries: &[Rc<EntryState>], indent: u8) -> Vec<ListItem<'_>> {
    let items: Vec<ListItem> = entries
        .iter()
        .flat_map(|v| match v.as_ref() {
            EntryState::Directory(v) => create_list_item(v, indent),
            EntryState::File(v) => once(file::create_list_item(v, indent)).collect(),
            EntryState::Symlink(v) => once(symlink::create_list_item(v, indent)).collect(),
        })
        .collect();
    items
}
