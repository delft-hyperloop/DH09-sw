mod app;
mod render;

use std::io::Stdout;
use ratatui::prelude::*;

use crate::tui::app::App;
use crate::MessageReceiver;

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub async fn tui_main(stream: MessageReceiver) -> anyhow::Result<()> {
    let mut terminal = ratatui::try_init()?;
    App::new(stream).run(&mut terminal).await?;
    ratatui::try_restore()?;
    Ok(())
}

#[inline]
pub fn timestamp() -> String { chrono::offset::Local::now().format("%H:%M:%S.%3f").to_string() }
