use std::collections::BTreeMap;
use std::time::Duration;

use crossterm::event::poll;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::{self};
use gslib::Datatype;
use gslib::Message;
use gslib::ProcessedData;
use ratatui::Frame;

use crate::tui::Tui;
use crate::MessageReceiver;

#[allow(dead_code)]
pub struct App {
    pub data: BTreeMap<Datatype, ProcessedData>,
    pub stream: MessageReceiver,
    pub scroll: usize,
    pub is_running: bool,
}

impl App {
    pub fn new(stream: MessageReceiver) -> Self {
        Self { data: Default::default(), stream, is_running: true, scroll: 0 }
    }

    pub async fn run(&mut self, terminal: &mut Tui) -> anyhow::Result<()> {
        while self.is_running {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.receive_data().await;
            self.handle_keyboard_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) { frame.render_widget(self, frame.area()); }

    async fn receive_data(&mut self) {
        while let Ok(datapoint) = self.stream.try_recv() {
            match datapoint {
                Message::Data(datapoint) => {
                    self.data.insert(datapoint.datatype, datapoint);
                },
                _ => { // todo: do something with other messages?
                },
            }
        }
    }

    fn handle_keyboard_events(&mut self) -> std::io::Result<()> {
        if poll(Duration::from_millis(1))? {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('c') => {
                            ratatui::restore();
                            std::process::exit(0);
                        },
                        KeyCode::Up | KeyCode::Char('k') => {
                            self.scroll = self.scroll.wrapping_add(1);
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            self.scroll = self.scroll.saturating_sub(1);
                        }
                        _ => {},
                    }
                },
                _ => {},
            };
        } else {
            // timeout expired
        }
        Ok(())
    }
}
