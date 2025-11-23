use crate::{
    state::diode::entry_state::EntryState,
    ui::explorer::{directory, file, symlink},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{List, ListItem, Paragraph, Widget},
};

pub struct ExplorerPane<'a> {
    pub list: List<'a>,
    pub info: Paragraph<'a>,
}

pub fn create_pane(entries: &[EntryState]) -> ExplorerPane<'_> {
    ExplorerPane {
        list: create_list(entries),
        info: create_info(entries),
    }
}

fn create_layout(area: Rect) -> [Rect; 2] {
    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(area);
    [rects[0], rects[1]]
}

fn create_list(entries: &[EntryState]) -> List<'_> {
    let items: Vec<ListItem> = entries
        .iter()
        .map(|v| match v {
            EntryState::Directory(dir) => directory::create_list_item(dir),
            EntryState::File(file) => file::create_list_item(file),
            EntryState::Symlink(symlink) => symlink::create_list_item(symlink),
        })
        .collect();
    List::new(items)
}

fn create_info(entries: &[EntryState]) -> Paragraph<'_> {
    let text = format!("{} items", entries.len());
    Paragraph::new(text)
}

impl Widget for ExplorerPane<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = create_layout(area);
        self.list.render(layout[0], buf);
        self.info.render(layout[1], buf);
    }
}
