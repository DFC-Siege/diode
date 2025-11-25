use color_eyre::Result;
use crossterm::event::{Event, EventStream, KeyEventKind};
use futures::{FutureExt, StreamExt};
use ratatui::DefaultTerminal;

use crate::{input_handling::input_handler, state::diode::diode_state::DiodeState, ui::app::draw};

#[derive(Debug)]
pub struct App {
    running: bool,
    event_stream: EventStream,
    pub diode_state: DiodeState,
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
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    input_handler::on_key_event(self, key)
                }
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
