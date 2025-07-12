#![allow(clippy::manual_flatten)]
use std::collections::BTreeMap;
use std::io::BufRead;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use crossterm::event::poll;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use crossterm::event::{self};
use gslib::Datatype;
use gslib::ProcessedData;
use ratatui::Frame;

use crate::Tui;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[allow(dead_code)]
pub struct App {
    pub data: BTreeMap<Datatype, ProcessedData>,
    pub scroll: usize,
    pub is_running: bool,
    pub stream: Receiver<ProcessedData>,
    pub input_mode: InputMode,
    pub cur_search: String,
    pub child: Child,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let (tx, rx) = std::sync::mpsc::channel();

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let gui_dir = manifest_dir.parent().expect("ooga booga").to_path_buf();

        eprintln!("Spawning GUI from: {}", gui_dir.display());

        let mut child = unsafe {
            Command::new("npm")
                .current_dir(&gui_dir)
                .args(["run", "gui"])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .pre_exec(|| {
                    // equivalent to C setsid()
                    nix::unistd::setsid().map_err(std::io::Error::other)?;
                    Ok(())
                })
                .spawn()
                .expect("Failed to spawn gui")
        };

        let map = BTreeMap::new();

        // thread to read stdout
        if let Some(out) = child.stdout.take() {
            let tx = tx.clone();
            thread::spawn(move || {
                let reader = std::io::BufReader::new(out);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if let Ok(Some(dp)) = read_datapoint(&line) {
                            let _ = tx.send(dp);
                        }
                    } else {
                        eprintln!("failed to read from child: {line:?}");
                    }
                }
            });
        }

        Ok(Self {
            data: map,
            is_running: true,
            scroll: 0,
            stream: rx,
            child,
            input_mode: InputMode::Normal,
            cur_search: String::new(),
        })
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
                    match self.input_mode {
                        InputMode::Normal => match key_event.code {
                            KeyCode::Char('q') | KeyCode::Char('c') => {
                                self.is_running = false;
                            },
                            KeyCode::Up | KeyCode::Char('k') => {
                                self.scroll = self.scroll.saturating_sub(1);
                            },
                            KeyCode::Down | KeyCode::Char('j') => {
                                self.scroll = self
                                    .scroll
                                    .wrapping_add(1)
                                    .min(self.data.len().saturating_sub(1));
                            },
                            KeyCode::Esc => self.cur_search = String::new(),
                            KeyCode::Char('/') => self.input_mode = InputMode::Editing,
                            _ => {},
                        },
                        InputMode::Editing => match key_event.code {
                            KeyCode::Esc => {
                                self.input_mode = InputMode::Normal;
                                self.cur_search = String::new();
                            },
                            KeyCode::Enter => {
                                self.input_mode = InputMode::Normal;
                            },
                            KeyCode::Backspace => {
                                self.cur_search.truncate(self.cur_search.len().saturating_sub(1))
                            },
                            KeyCode::Char('c')
                                if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                            {
                                self.is_running = false;
                            },
                            KeyCode::Char(c) => {
                                self.cur_search.push(c);
                            },
                            KeyCode::Up => {
                                self.scroll = self.scroll.saturating_sub(1);
                            },
                            KeyCode::Down => {
                                self.scroll = self
                                    .scroll
                                    .wrapping_add(1)
                                    .min(self.data.len().saturating_sub(1));
                            },
                            _ => {},
                        },
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
