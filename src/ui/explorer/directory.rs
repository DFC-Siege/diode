use crate::{
    state::diode::{directory_state::DirectoryState, entry_state::EntryState},
    ui::explorer::{file, symlink},
};
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::ListItem,
};
use std::{iter::once, rc::Rc};

pub fn create_list_item(directory: &DirectoryState, indent: u8) -> Vec<ListItem<'_>> {
    let mut items: Vec<ListItem> = Vec::new();
    let tabs = "  ".repeat(indent as usize);
    let mut item = match directory.collapsed {
        true => ListItem::new(format!("{}📁 {}", tabs, directory.name.to_string_lossy())),
        false => ListItem::new(format!("{}📂 {}", tabs, directory.name.to_string_lossy())),
    };

    if directory.selected.get() {
        item = item.style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    }

    items.push(item);
    items.append(&mut create_list(&directory.entries, indent + 1));
    items
}

fn create_list(entries: &[Rc<EntryState>], indent: u8) -> Vec<ListItem<'_>> {
    entries
        .iter()
        .flat_map(|v| match v.as_ref() {
            EntryState::Directory(v) => create_list_item(v, indent),
            EntryState::File(v) => once(file::create_list_item(v, indent)).collect(),
            EntryState::Symlink(v) => once(symlink::create_list_item(v, indent)).collect(),
        })
        .collect()
}
