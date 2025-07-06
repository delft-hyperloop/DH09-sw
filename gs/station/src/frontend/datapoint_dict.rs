use std::fmt::Formatter;

use gslib::Datapoint;
use gslib::Datatype;

const DATATYPE_HEADER: &str = "Datatype";
const VALUE_HEADER: &str = "Value";
const TIMESTAMP_HEADER: &str = "Timestamp";

pub struct DatapointDict {
    datapoints: Vec<Datapoint>,
    max_length_type: usize,
    max_length_value: usize,
    terminal_command_rx: TerminalCommandReceiver,
    scrolled_rows: usize,
}

impl DatapointDict {
    pub fn new(terminal_command_rx: TerminalCommandReceiver) -> Self {
        Self {
            datapoints: Vec::new(),
            max_length_type: 30,
            max_length_value: 10,
            terminal_command_rx,
            scrolled_rows: 0,
        }
    }
    
    /// Processes the commands received.
    pub async fn process_command(&mut self) {
        if !self.terminal_command_rx.is_empty() {
            match self.terminal_command_rx.recv().await {
                Some(TerminalCommands::Up) if self.scrolled_rows != 0 => {
                    self.scrolled_rows -= 1
                }
                Some(TerminalCommands::Down) if self.scrolled_rows != self.datapoints.len() => self.scrolled_rows += 1,
                _ => {}
            }
        }
    }

    /// Formats the dictionary as a string
    pub fn as_string(&self) -> String {
        let mut result = format!(
            "{:<width_types$}|{:<width_values$}|{}\n\r{}",
            DATATYPE_HEADER,
            VALUE_HEADER,
            TIMESTAMP_HEADER,
            "-".repeat(self.max_length_value + self.max_length_type + TIMESTAMP_HEADER.len() + 2),
            width_types = self.max_length_type,
            width_values = self.max_length_value
        );
        
        for i in self.scrolled_rows..self.datapoints.len() {
            let dp = self.datapoints[i];
            result.push_str(&format!(
                "\n\r{:<width_types$}|{:<width_values$}|{}",
                format!("{:?}", dp.datatype),
                dp.value,
                dp.timestamp,
                width_types = self.max_length_type,
                width_values = self.max_length_value
            ));
        }
        result.push_str("\n\r");
        result
    }

    pub fn add_datapoint(&mut self, datapoint: Datapoint) {
        if datapoint.datatype == Datatype::ConfigHash
            || datapoint.datatype == Datatype::DataHash
            || datapoint.datatype == Datatype::CommandHash
        {
            return;
        }
        for i in 0..self.datapoints.len() {
            if self.datapoints[i].datatype == datapoint.datatype {
                self.datapoints[i] = datapoint;
                self.max_length_value = std::cmp::max(
                    self.max_length_value,
                    format!("{:?}", datapoint.value).len(),
                );
                self.max_length_type = std::cmp::max(
                    self.max_length_type,
                    format!("{:?}", datapoint.datatype).len(),
                );
                return;
            }
        }
        self.datapoints.push(datapoint);
    }
}

impl std::fmt::Display for DatapointDict {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.as_string()) }
}

pub enum TerminalCommands {
    Up,
    Down,
}

pub type TerminalCommandReceiver = tokio::sync::mpsc::Receiver<TerminalCommands>;
pub type TerminalCommandSender = tokio::sync::mpsc::Sender<TerminalCommands>;
