use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Text},
    widgets::{ Block, List, ListItem, ListState, Paragraph },
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.size());

    match app.current_screen {
        CurrentScreen::StoriesList => {
            let stories = &app.current_stories.data;
            let mut list_items: Vec<ListItem> = vec![];
            for story in stories.into_iter() {
                list_items.push(ListItem::new(story.name.to_string()))
            }
            let body_block = Block::default()
                .style(Style::default());
            let mut state = ListState::default().with_selected(Some(app.selected_index));
            let body = List::new(list_items)
                .block(body_block)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().bg(Color::White).fg(Color::Black));
            frame.render_stateful_widget(body, layout[0], &mut state);

            let footer_block = Block::default()
                .style(Style::default());
            let footer = Paragraph::new(Text::styled(
                format!("cutty 0.1.0 | @{}", app.current_member.mention_name),
                Style::default().fg(Color::LightBlue),
            ))
            .block(footer_block);
            frame.render_widget(footer, layout[1]);
        }
        CurrentScreen::StoryDetail => {
            let footer_block = Block::default()
                .style(Style::default());
            let footer = Paragraph::new(Text::styled(
                format!("cutty 0.1.0 | @{}", app.current_member.mention_name),
                Style::default().fg(Color::LightBlue),
            ))
            .block(footer_block);
            frame.render_widget(footer, layout[1]);
        }
    }
}

