use crate::{
    state::diode::{entry_state::EntryState, explorer_state::ExplorerState},
    ui::explorer::{directory, file},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};
use std::path::Path;

pub struct ExplorerPane {
    pub list: List<'static>,
    pub info: Paragraph<'static>,
    pub selected: bool,
}

#[derive(Debug)]
pub struct ExplorerPaneState {
    pub list_state: ListState,
}

impl ExplorerPaneState {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
        }
    }
}

pub fn create_pane(
    explorer_state: &ExplorerState,
    selected: bool,
    base_path: &Path,
) -> ExplorerPane {
    let entries: Vec<&EntryState> = explorer_state
        .get_visible_entries()
        .map(|(_, v)| v)
        .collect();

    ExplorerPane {
        list: create_list(&entries, base_path),
        info: create_info(explorer_state),
        selected,
    }
}

fn create_layout(area: Rect) -> [Rect; 2] {
    let rects = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
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

fn create_info(explorer_state: &ExplorerState) -> Paragraph<'static> {
    if let Some(entry) = explorer_state.get_selected_entry() {
        let text = format!("Title: {}", entry.name().to_string_lossy());
        Paragraph::new(text)
    } else {
        let text = format!("{} items", explorer_state.entries.len());
        Paragraph::new(text)
    }
}

impl StatefulWidget for ExplorerPane {
    type State = ExplorerPaneState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
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
        StatefulWidget::render(self.list, layout[0], buf, &mut state.list_state);
        self.info.render(layout[1], buf);
    }
}
