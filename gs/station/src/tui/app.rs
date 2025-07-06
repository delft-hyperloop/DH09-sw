use std::collections::BTreeMap;

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
    pub is_running: bool,
}

impl App {
    pub fn new(stream: MessageReceiver) -> Self {
        Self { data: Default::default(), stream, is_running: true }
    }

    pub async fn run(&mut self, terminal: &mut Tui) -> anyhow::Result<()> {
        while self.is_running {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.receive_data().await;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) { frame.render_widget(self, frame.size()); }

    async fn receive_data(&mut self) {
        match self.stream.recv().await {
            Ok(Message::Data(datapoint)) => {
                self.data.insert(datapoint.datatype, datapoint);
            },
            Ok(_) => { // todo: do something with other messages?
            },
            Err(e) => {
                eprintln!("error blocking_recv in TUI: {e:?}");
            },
        }
    }
}
