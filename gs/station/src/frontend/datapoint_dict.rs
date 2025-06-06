use std::fmt::Formatter;
use std::io::Stdout;
use std::io::{self};
use std::time::Duration;

use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event as CEvent;
use crossterm::event::KeyCode;
use crossterm::event::{self};
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use gslib::Datapoint;
use gslib::Datatype;
use ratatui::backend::Backend;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::widgets::Cell;
use ratatui::widgets::Row;
use ratatui::widgets::Table;
use ratatui::Terminal;

const DATATYPE_HEADER: &str = "Datatype";
const VALUE_HEADER: &str = "Value";
const TIMESTAMP_HEADER: &str = "Timestamp";

pub struct DatapointDict {
    datapoints: [Datapoint; 50],
    size: usize,
    capacity: usize,
    max_length_type: usize,
    max_length_value: usize,
    display: Terminal<CrosstermBackend<Stdout>>,
}

impl DatapointDict {
    pub fn new(capacity: usize) -> Self {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        Self {
            datapoints: [Datapoint::new(Datatype::DefaultDatatype, 0, 0); 50],
            size: 0,
            capacity,
            max_length_type: DATATYPE_HEADER.len(),
            max_length_value: VALUE_HEADER.len(),
            display: terminal,
        }
    }

    pub fn stop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.display.backend_mut(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
        self.display.show_cursor().unwrap();
    }

    pub fn as_string(&self) -> String {
        let mut result = format!(
            "{}{}|{}{}|{}\n{}",
            DATATYPE_HEADER,
            " ".repeat(self.max_length_type.saturating_sub(DATATYPE_HEADER.len())),
            VALUE_HEADER,
            " ".repeat(self.max_length_value.saturating_sub(VALUE_HEADER.len())),
            TIMESTAMP_HEADER,
            "-".repeat(self.max_length_value + self.max_length_type + TIMESTAMP_HEADER.len() + 2)
        );
        for i in 0..self.size {
            let dp = self.datapoints[i];
            result.push_str(&format!(
                "\n{:?}{}|{}{}|{}",
                dp.datatype,
                " ".repeat(self.max_length_type.saturating_sub(format!("{:?}", dp.datatype).len())),
                dp.value,
                " ".repeat(self.max_length_value.saturating_sub(format!("{:?}", dp.value).len())),
                dp.timestamp,
            ));
        }
        result.push_str("\n");
        result
    }

    pub fn add_datapoint(&mut self, datapoint: Datapoint) {
        if datapoint.datatype != Datatype::CommandHash
            && datapoint.datatype != Datatype::EventsHash
            && datapoint.datatype != Datatype::DataHash
            && datapoint.datatype != Datatype::ConfigHash
            && datapoint.datatype != Datatype::DefaultDatatype
        {
            for i in 0..self.capacity {
                if self.datapoints[i].datatype == Datatype::DefaultDatatype {
                    self.max_length_value = std::cmp::max(
                        self.max_length_value,
                        format!("{:?}", datapoint.value).len(),
                    );
                    self.max_length_type = std::cmp::max(
                        self.max_length_type,
                        format!("{:?}", datapoint.datatype).len(),
                    );
                    self.datapoints[i] = datapoint;
                    self.size += 1;
                    return;
                } else if datapoint.datatype == self.datapoints[i].datatype {
                    self.datapoints[i] = datapoint;
                    return;
                }
            }
        }
        // TODO: replace oldest?
        self.draw();
    }

    /// draw table into the given frame.
    fn draw(&mut self) -> () {
        self.display.draw(|f| {
            let size = f.size();

            // Prepare rows: each row is three cells.
            let rows: Vec<Row> = self
                .datapoints
                .iter()
                .map(|row_data| {
                    let cells = [
                        Cell::from(row_data.datatype.unit()),
                        Cell::from(row_data.value.to_string()),
                        Cell::from(row_data.timestamp.to_string()),
                    ];
                    Row::new(cells)
                })
                .collect();

            // Build the table with three equally‐spaced columns.
            let table = Table::new(
                rows,
                &[
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ],
            )
            .header(
                Row::new(
                    [DATATYPE_HEADER, VALUE_HEADER, TIMESTAMP_HEADER]
                        .iter()
                        .map(|h| Cell::from(*h))
                        .collect::<Vec<Cell>>(),
                )
                .style(Style::default().add_modifier(Modifier::BOLD)),
            );

            // Center the table in the full frame.
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            f.render_widget(table, chunks[0]);
        }).unwrap();
    }
}

impl std::fmt::Display for DatapointDict {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = &mut format!("\x1B[{}A{}", self.capacity + 2, self.as_string());
        for _ in 0..(self.capacity - self.size) {
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}
