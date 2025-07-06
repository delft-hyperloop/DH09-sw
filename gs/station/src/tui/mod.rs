mod app;
mod render;

use std::io::stdout;
use std::io::Stdout;
use std::io::{self};

use crossterm::terminal::*;
use ratatui::prelude::*;
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;

use crate::tui::app::App;
use crate::MessageReceiver;

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    // alternate screen = full-screen tui. this is optional
    crossterm::execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    // register SIGINT
    {
        let mut signals = Signals::new([SIGINT])?;
        std::thread::spawn(move || {
            for sig in signals.forever() {
                if sig == SIGINT {
                    // drop the result since we're quitting anyway
                    let _ = restore();
                    std::process::exit(0);
                }
            }
        });
    }

    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    // same as above
    crossterm::execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub async fn tui_main(stream: MessageReceiver) {
    let mut terminal = init().unwrap(); // initialise the crossterm magic
    App::new(stream).run(&mut terminal).await.unwrap();
    restore().unwrap(); // restore the terminal, it gets messed up after the TUI
}

#[inline]
pub fn timestamp() -> String { chrono::offset::Local::now().format("%H:%M:%S.%3f").to_string() }
