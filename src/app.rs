use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use ratatui::DefaultTerminal;

use crate::{state::diode::diode_state::DiodeState, ui::app::draw};

#[derive(Debug)]
pub struct App {
    running: bool,
    event_stream: EventStream,
    diode_state: DiodeState,
}

impl App {
    pub fn new(diode_state: DiodeState) -> Self {
        Self {
            running: false,
            event_stream: EventStream::new(),
            diode_state,
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|v| draw(v, &self.diode_state))?;
            self.handle_crossterm_events().await?;
        }
        Ok(())
    }

    async fn handle_crossterm_events(&mut self) -> Result<()> {
        let event = self.event_stream.next().fuse().await;
        if let Some(Ok(evt)) = event {
            match evt {
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
