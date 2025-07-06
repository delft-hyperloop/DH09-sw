use gslib::{Datatype, ProcessedData};
use ratatui::prelude::*;
use ratatui::widgets::block::*;
use ratatui::widgets::*;

use crate::tui::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // first get the rows we should render
        let mut entries: Vec<(&Datatype, &ProcessedData)> = self
            .data
            .iter()
            .collect();
        entries.sort_by_key(|(d, _)| d.unit());

        let columns = if area.width >= 80 { 2 } else { 1 };
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                std::iter::repeat_n(Constraint::Percentage(100 / columns as u16), columns)
                    .collect::<Vec<_>>(),
            )
            .split(area);

        let chunk_size = entries.len().div_ceil(columns);
        for (i, chunk_area) in chunks.iter().enumerate() {
            let slice = &entries[i * chunk_size..std::cmp::min((i+1)*chunk_size, entries.len())];
            let rows = slice.iter().map(|(dt, val)| {
                Row::new(vec![dt.unit().to_string(), val.value.to_string()])
            });

            let table_col_widths = vec![Constraint::Length(5), Constraint::Length(5), Constraint::Length(5)];
            let table = Table::new(rows, table_col_widths)
                .header(Row::new(vec!["Key", "Value"]))
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Entries ({}/{})", i+1, columns)))
                .widths([Constraint::Percentage(50), Constraint::Percentage(50)])
                .column_spacing(1)
                .highlight_symbol(">>");

            ratatui::widgets::Widget::render(table, *chunk_area, buf);
        }
    }
}
