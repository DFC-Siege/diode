use std::{iter::once, rc::Rc};

use crate::{
    state::diode::entry_state::EntryState,
    ui::explorer::{directory, file, symlink},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

pub struct ExplorerPane<'a> {
    pub list: List<'a>,
    pub info: Paragraph<'a>,
    pub selected: bool,
}

pub fn create_pane(entries: &[Rc<EntryState>], selected: bool) -> ExplorerPane<'_> {
    ExplorerPane {
        list: create_list(entries),
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

fn create_list(entries: &[Rc<EntryState>]) -> List<'_> {
    let items: Vec<ListItem> = entries
        .iter()
        .flat_map(|v| match v.as_ref() {
            EntryState::Directory(dir) => directory::create_list_item(dir, 0),
            EntryState::File(file) => once(file::create_list_item(file, 0)).collect(),
            EntryState::Symlink(symlink) => once(symlink::create_list_item(symlink, 0)).collect(),
        })
        .collect();
    List::new(items)
}

fn create_info(entries: &[Rc<EntryState>]) -> Paragraph<'_> {
    let text = format!("{} items", entries.len());
    Paragraph::new(text)
}

impl Widget for ExplorerPane<'_> {
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
