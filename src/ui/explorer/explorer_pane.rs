use std::path::Path;

use crate::{
    state::diode::entry_state::EntryState,
    ui::explorer::{directory, file},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

pub struct ExplorerPane {
    pub list: List<'static>,
    pub info: Paragraph<'static>,
    pub selected: bool,
}

pub fn create_pane(entries: &[&EntryState], selected: bool, base_path: &Path) -> ExplorerPane {
    ExplorerPane {
        list: create_list(entries, base_path),
        info: create_info(entries),
        selected,
    }
}

fn create_layout(area: Rect) -> [Rect; 2] {
    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(area);
    [rects[0], rects[1]]
}

fn create_list(entries: &[&EntryState], base_path: &Path) -> List<'static> {
    let items: Vec<ListItem> = entries
        .iter()
        .flat_map(|v| match v {
            EntryState::Directory(dir) => directory::create_list_item(dir, v.get_indent(base_path)),
            EntryState::File(file) => vec![file::create_list_item(file, v.get_indent(base_path))],
        })
        .collect();
    List::new(items)
}

fn create_info(entries: &[&EntryState]) -> Paragraph<'static> {
    let text = format!("{} items", entries.len());
    Paragraph::new(text)
}

impl Widget for ExplorerPane {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_color = if self.selected {
            Color::Cyan
        } else {
            Color::DarkGray
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));
        let inner = block.inner(area);
        block.render(area, buf);
        let layout = create_layout(inner);
        self.list.render(layout[0], buf);
        self.info.render(layout[1], buf);
    }
}
