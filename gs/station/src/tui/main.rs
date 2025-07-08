#![cfg(feature = "tui")]
mod app;
mod render;

use std::io::Stdout;
use ratatui::prelude::*;

use crate::app::App;

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn tui_main() -> anyhow::Result<()> {
    let mut terminal = ratatui::try_init()?;
    App::new().run(&mut terminal)?;
    ratatui::try_restore()?;
    Ok(())
}

#[inline]
pub fn timestamp() -> String { chrono::offset::Local::now().format("%H:%M:%S.%3f").to_string() }

fn main() -> anyhow::Result<()> {
    println!("starting tui");
    tui_main()?;
    println!("tui exiting");
    Ok(())
}
