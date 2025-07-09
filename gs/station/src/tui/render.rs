use gslib::Datatype;
use gslib::ProcessedData;
use ratatui::prelude::*;
use ratatui::widgets::block::*;
use ratatui::widgets::*;

use crate::app::App;
use crate::app::InputMode;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // split top and bottom for content & search bar

        let [content_area, input_area] =
            Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).areas(area);

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
                    val.timestamp.to_string(),
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
    }
}
