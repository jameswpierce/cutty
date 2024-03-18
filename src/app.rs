use std::io::{self, stdout, Stdout};
use crate::shortcut::{get_member, MemberInfo, search_stories, StorySearchResults};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crossterm::{
    event::{
        self,
        Event,
        KeyCode,
        KeyEvent,
        KeyEventKind,
    },
};
use crate::{
    ui::ui,
};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub enum CurrentScreen {
    StoriesList,
    StoryDetail,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_member: MemberInfo,
    pub current_stories: StorySearchResults,
    pub selected_index: usize,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::StoriesList,
            current_member: get_member(),
            current_stories: search_stories(),
            selected_index: 0,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui(frame, self))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.current_screen {
            CurrentScreen::StoriesList => match key_event.code {
                KeyCode::Char('q') => {
                    self.exit();
                }
                KeyCode::Char('j') => {
                    self.next_story();
                }
                KeyCode::Char('k') => {
                    self.prev_story();
                }
                KeyCode::Enter => {
                    self.current_screen = CurrentScreen::StoryDetail;
                }
                _ => {}
            }
            CurrentScreen::StoryDetail => match key_event.code {
                KeyCode::Esc => {
                    self.current_screen = CurrentScreen::StoriesList;
                }
                _ => {}
            }
        };
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn next_story(&mut self) {
        self.selected_index += 1;
    }

    fn prev_story(&mut self) {
        self.selected_index -= 1;
    }
}
