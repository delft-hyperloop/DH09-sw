#![cfg(feature = "tui")]
mod app;
mod render;

use std::io::stdout;
use std::io::Stdout;
use std::process::Child;

use crossterm::execute;
use nix::libc::killpg;
use nix::unistd::Pid;
use ratatui::prelude::*;

use crate::app::App;

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn tui_main() -> anyhow::Result<()> {
    let mut terminal = ratatui::try_init()?;
    let mut app = App::new()?;
    app.run(&mut terminal)?;
    ratatui::try_restore()?;
    let _ = execute!(stdout(), crossterm::cursor::Show);
    kill_tree(&mut app.child);
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

fn kill_tree(child: &mut Child) {
    let pgid = Pid::from_raw(child.id() as i32);
    // send SIGTERM to the whole group
    unsafe {
        killpg(pgid.as_raw(), 15 /* sigterm */)
    };
    // wait a bit and SIGKILL
    std::thread::sleep(std::time::Duration::from_millis(100));
    unsafe {
        killpg(pgid.as_raw(), 9 /* sigkill */)
    };
    // now reap the root child.
    // this will also reap zombies of its children
    let _ = child.wait();
}
