use crate::state::diode::directory_state::DirectoryState;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::ListItem,
};

pub fn create_list_item(directory: &DirectoryState, indent: u8) -> Vec<ListItem<'static>> {
    let mut items: Vec<ListItem> = Vec::new();
    let tabs = "  ".repeat(indent as usize);
    let mut item = match directory.collapsed {
        true => ListItem::new(format!(
            "{}📁 {}",
            tabs,
            directory.directory.name.to_string_lossy()
        )),
        false => ListItem::new(format!(
            "{}📂 {}",
            tabs,
            directory.directory.name.to_string_lossy()
        )),
    };

    if directory.marked {
        item = item.style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );
    }

    if directory.selected {
        item = item.style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    }

    items.push(item);
    items
}
