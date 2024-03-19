use std::io::{self, stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod shortcut;
mod ui;
use crate::app::{App, Tui};

fn main() -> io::Result<()> {
    let mut terminal = init()?;
    let mut app = App::new();
    let res = app.run(&mut terminal);
    restore(&mut terminal)?;
    res?;
    Ok(())
}

fn init() -> io::Result<Tui> {
    let backend = CrosstermBackend::new(stdout());
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    Terminal::new(backend)
}

fn restore(terminal: &mut Tui) -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}
