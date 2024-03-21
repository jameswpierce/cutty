use crate::shortcut::{
    get_current_member, list_workflows, search_epics, search_stories, EpicSearchResults, MemberInfo,
    StorySearchResults, Workflow,
};
use std::io::{self, Stdout};

use ratatui::{backend::CrosstermBackend, Terminal};

use crate::ui::ui;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub enum CurrentScreen {
    EpicsList,
    StoriesList,
    StoryDetail,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_epics: EpicSearchResults,
    pub current_member: MemberInfo,
    pub current_stories: StorySearchResults,
    pub workflows: Vec<Workflow>,
    pub scroll: u16,
    pub selected_index: usize,
    pub story_details_workflow_state_change: bool,
    pub exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::StoriesList,
            current_epics: search_epics(),
            current_member: get_current_member(),
            current_stories: search_stories(),
            workflows: list_workflows(),
            scroll: 0,
            selected_index: 0,
            story_details_workflow_state_change: false,
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
            CurrentScreen::EpicsList => match key_event.code {
                KeyCode::Char('q') => {
                    self.exit();
                }
                KeyCode::Char('j') => {
                    self.next_list_item();
                }
                KeyCode::Char('k') => {
                    self.prev_list_item();
                }
                KeyCode::Char('r') => {
                    self.current_epics = search_epics();
                }
                KeyCode::Char('s') => {
                    self.selected_index = 0;
                    self.current_screen = CurrentScreen::StoriesList;
                }
                KeyCode::Enter => {
                    self.current_screen = CurrentScreen::StoryDetail;
                }
                _ => {}
            },
            CurrentScreen::StoriesList => match key_event.code {
                KeyCode::Char('q') => {
                    self.exit();
                }
                KeyCode::Char('j') => {
                    self.next_list_item();
                }
                KeyCode::Char('k') => {
                    self.prev_list_item();
                }
                KeyCode::Char('r') => {
                    self.current_stories = search_stories();
                }
                KeyCode::Char('e') => {
                    self.selected_index = 0;
                    self.current_screen = CurrentScreen::EpicsList;
                }
                KeyCode::Enter => {
                    self.current_screen = CurrentScreen::StoryDetail;
                }
                _ => {}
            },
            CurrentScreen::StoryDetail => match key_event.code {
                KeyCode::Char('b') => {
                    self.scroll = 0;
                    self.current_screen = CurrentScreen::StoriesList;
                }
                KeyCode::Char('j') => {
                    self.scroll += 1;
                }
                KeyCode::Char('k') => {
                    if self.scroll > 0 {
                        self.scroll -= 1;
                    }
                }
                KeyCode::Char('s') => {}
                _ => {}
            },
        };
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn next_list_item(&mut self) {
        if self.selected_index < (self.current_stories.data.len() - 1) {
            self.selected_index += 1;
        }
    }

    fn prev_list_item(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}
