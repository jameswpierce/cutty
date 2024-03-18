use std::io::{self, stdout, Stdout};

use crossterm::{
    execute,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    event::{
        EnableMouseCapture,
        DisableMouseCapture,
    },
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod ui;
mod shortcut;
use crate::{
    app::{App, Tui},
};



fn main() -> io::Result<()> {
    let mut terminal = init()?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    restore(&mut terminal)?;
    res?;
    Ok(())
}

fn run_app(
    terminal: &mut Tui,
    app: &mut App,
) -> io::Result<bool> {
    loop {
        app.run(terminal);
    }
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
