use gslib::Datatype;
use gslib::ProcessedData;
use ratatui::prelude::*;
use ratatui::widgets::block::*;
use ratatui::widgets::*;

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // first get the rows we should render
        let mut entries: Vec<(&Datatype, &ProcessedData)> = self.data.iter().collect();
        entries.sort_by_key(|(d, _)| d.unit());
        let count = entries.len();
        if self.scroll != 0 && !entries.is_empty() {
            entries.rotate_left(self.scroll % count);
        }

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
            let slice =
                &entries[i * chunk_size..std::cmp::min((i + 1) * chunk_size, entries.len())];
            let longest_name =
                slice.iter().map(|(_, x)| format!("{x:?}").chars().count()).max().unwrap_or(0);
            let rows = slice.iter().map(|(dt, val)| {
                Row::new(vec![
                    format!("{dt:?}"),
                    format!("{:.3} ({})", val.value, dt.unit()),
                    val.timestamp.to_string(),
                ])
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
    }
}
