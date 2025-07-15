use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use chrono::DateTime;
use gslib::Datatype;
use gslib::ProcessedData;
use ratatui::prelude::*;
use ratatui::symbols::Marker;
use ratatui::widgets::block::*;
use ratatui::widgets::*;

use crate::app::App;
use crate::app::InputMode;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // split top and bottom for content & search bar

        let [content_area, commands_area, stats_area, input_area] = match self.input_mode {
            InputMode::Editing => Layout::vertical([
                Constraint::Min(10),
                Constraint::Length(5),
                Constraint::Length(8),
                Constraint::Length(3),
            ])
            .areas(area),
            InputMode::Normal => Layout::vertical([
                Constraint::Min(10),
                Constraint::Length(6),
                Constraint::Length(8),
                Constraint::Length(1),
            ])
            .areas(area),
        };

        // first get the rows we should render
        let mut entries: Vec<(&Datatype, &ProcessedData)> = self.data.iter().collect();
        entries.sort_by_key(|(d, _)| d.unit());
        let count = entries.len();
        if self.scroll != 0 && !entries.is_empty() {
            entries.rotate_left(self.scroll % count);
        }

        let columns = if area.width >= 80 { area.width as usize / 42 } else { 1 };
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                std::iter::repeat_n(Constraint::Percentage(100 / columns as u16), columns)
                    .collect::<Vec<_>>(),
            )
            .split(content_area);

        let chunk_size = entries.len().div_ceil(columns);
        for (i, chunk_area) in chunks.iter().enumerate() {
            let slice = if i * chunk_size < entries.len() {
                &entries[i * chunk_size..std::cmp::min((i + 1) * chunk_size, entries.len())]
            } else {
                &[]
            };
            let longest_name =
                slice.iter().map(|(_, x)| format!("{x:?}").chars().count()).max().unwrap_or(0);
            let rows = slice.iter().map(|(dt, val)| {
                let key = format!("{dt:?}");
                let row = Row::new(vec![
                    key.clone(),
                    format!("{:.3} ({})", val.value, dt.unit()),
                    val.style.clone(),
                ]);
                if !key.is_empty()
                    && !self.cur_search.is_empty()
                    && key.to_lowercase().contains(&self.cur_search)
                {
                    row.on_yellow().black()
                } else {
                    row
                }
            });

            let table_col_widths = vec![
                Constraint::Min(longest_name as u16 + 1),
                Constraint::Min(13),
                Constraint::Min(10),
            ];
            let table = Table::new(rows, table_col_widths)
                .header(Row::new(vec!["Key", "Value", "timestamp"]))
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "Entries ({}/{})",
                    i + 1,
                    columns
                )))
                .column_spacing(1)
                .highlight_symbol(">>");

            ratatui::widgets::Widget::render(table, *chunk_area, buf);
        }

        // now render the search bar
        let mut sb = Block::new().borders(Borders::ALL).title("press / to search");

        let search = Line::from(self.cur_search.clone());
        if self.input_mode == InputMode::Editing {
            sb = sb.border_style(Style::default().yellow());
        }
        let inner = sb.inner(input_area);
        sb.render(input_area, buf);
        search.render(inner, buf);

        let mut commands_list = self
            .commands
            .iter()
            .map(|(x, y)| (x.to_string(), y.to_string()))
            .collect::<Vec<(String, String)>>();

        commands_list.sort_by_key(|(_, x)| x.clone());

        let longest = commands_list.iter().map(|x| x.0.len() + x.1.len()).max().unwrap_or(40);

        let columns = if area.width as usize >= (2 * (longest + 4)) {
            area.width as usize / (longest + 4)
        } else {
            1
        };
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                std::iter::repeat_n(Constraint::Percentage(100 / columns as u16), columns)
                    .collect::<Vec<_>>(),
            )
            .split(commands_area);
        for (i, chunk_area) in chunks.iter().enumerate() {
            let slice = if i * chunk_size < commands_list.len() {
                &commands_list
                    [i * chunk_size..std::cmp::min((i + 1) * chunk_size, commands_list.len())]
            } else {
                &[]
            };

            let rows = slice.iter().map(|(key, t)| {
                let row = Row::new(vec![key.clone(), t.clone()]);
                if !key.is_empty()
                    && !self.cur_search.is_empty()
                    && key.to_lowercase().contains(&self.cur_search)
                {
                    row.on_yellow().black()
                } else {
                    row
                }
            });

            let table_col_widths = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
            let table = Table::new(rows, table_col_widths)
                // .header(Row::new(vec!["command", "timestamp"]))
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "commands sent to pod ({}/{})",
                    i + 1,
                    columns
                )))
                .column_spacing(1)
                .highlight_symbol(">>");

            ratatui::widgets::Widget::render(table, *chunk_area, buf);
        }

        // Compute current time in seconds since UNIX epoch
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();

        // Determine time bounds: from one minute ago to now
        let x_bounds = [now - 60.0, now];

        // Number of samples
        let n = self.kbps.len();
        let dt = if n > 1 { 60.0 / (n as f64 - 1.0) } else { 0.0 };

        // Build two datasets: .0 and .1 values over time
        let data0: Vec<(f64, f64)> = self
            .kbps
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &(v0, _v1))| (x_bounds[0] + i as f64 * dt, v0))
            .collect();
        let data1: Vec<(f64, f64)> = self
            .kbps
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &(_v0, v1))| (x_bounds[0] + i as f64 * dt, v1))
            .collect();

        // Determine y-axis bounds by inspecting all values, with a small margin
        let all_vals = self.kbps.iter().rev().flat_map(|&(v0, v1)| vec![v0, v1]);
        let (y_min, y_max) = all_vals
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), v| (min.min(v), max.max(v)));
        let y_margin = (y_max - y_min) * 0.05;
        let y_bounds = [0.0, y_max + y_margin];

        // Create the chart with two lines
        let chart = Chart::new(
            [
                Dataset::default()
                    .name("incoming data rate")
                    .marker(Marker::HalfBlock)
                    .style(Style::default().fg(Color::Blue))
                    .data(&data0),
                Dataset::default()
                    .name("outgoing data rate")
                    .marker(Marker::Bar)
                    .style(Style::default().fg(Color::Red))
                    .data(&data1),
            ]
            .to_vec(),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("kbps over last minute, blue:pod->gs, red:gs->pod"),
        )
        .x_axis(
            Axis::default().bounds(x_bounds).labels(
                [
                    // Label start and end times in HH:MM:SS
                    format!(
                        "{}",
                        DateTime::from_timestamp(x_bounds[0] as i64, 0).unwrap().format("%H:%M:%S")
                    ),
                    format!(
                        "{}",
                        DateTime::from_timestamp(x_bounds[1] as i64, 0).unwrap().format("%H:%M:%S")
                    ),
                ]
                .iter()
                .cloned(),
            ),
        )
        .y_axis(
            Axis::default().bounds(y_bounds).labels(
                [
                    format!("{:.2}", y_bounds[0]),
                    format!("{:.2}", (y_bounds[0] + y_bounds[1]) / 2.0),
                    format!("{:.2}", y_bounds[1]),
                ]
                .iter()
                .cloned(),
            ),
        );

        // Render the chart widget into the buffer
        chart.render(stats_area, buf);
    }
}
