#![allow(clippy::manual_flatten)]
use std::collections::BTreeMap;
use std::io::stdin;
use std::io::stdout;
use std::io::BufRead;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use crossterm::event::poll;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::{self};
use crossterm::execute;
use gslib::Datatype;
use gslib::ProcessedData;
use ratatui::Frame;

use crate::Tui;

#[allow(dead_code)]
pub struct App {
    pub data: BTreeMap<Datatype, ProcessedData>,
    pub scroll: usize,
    pub is_running: bool,
    pub stream: Receiver<ProcessedData>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let stdin = stdin();
            for line in stdin.lock().lines() {
                if let Ok(line) = line {
                    if let Ok(Some(dp)) = read_datapoint(&line) {
                        let _ = tx.send(dp);
                    }
                }
            }
        });
        Self { data: Default::default(), is_running: true, scroll: 0, stream: rx }
    }

    pub fn run(&mut self, terminal: &mut Tui) -> anyhow::Result<()> {
        while self.is_running {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.receive_data();
            self.handle_keyboard_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) { frame.render_widget(self, frame.area()); }

    fn receive_data(&mut self) {
        while let Ok(dp) = self.stream.try_recv() {
            self.data.insert(dp.datatype, dp);
        }
    }

    fn handle_keyboard_events(&mut self) -> std::io::Result<()> {
        if poll(Duration::from_micros(2000))? {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('c') => {
                            ratatui::restore();
                            let _ = execute!(stdout(), crossterm::cursor::Show);
                            std::process::exit(0);
                        },
                        KeyCode::Up | KeyCode::Char('k') => {
                            self.scroll = self.scroll.wrapping_add(1);
                        },
                        KeyCode::Down | KeyCode::Char('j') => {
                            self.scroll = self.scroll.saturating_sub(1);
                        },
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

fn read_datapoint(s: &str) -> anyhow::Result<Option<ProcessedData>> {
    if s.contains("datapoint:") {
        let parts = s.split(':').collect::<Vec<_>>();
        // structure: `datapoint:name:value:timestamp\n`
        let datatype = Datatype::from_str(parts[1]);
        let value: f64 = parts[2].parse()?;
        let timestamp: u64 = parts[3].parse()?;
        Ok(Some(ProcessedData {
            datatype,
            value,
            timestamp,
            style: "black".into(),
            units: datatype.unit(),
            lower: None,
            upper: None, // todo: get & display limits in tui?
        }))
    } else {
        Ok(None)
    }
}
