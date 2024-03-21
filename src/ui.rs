use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Line,
    style::{Color, Style},
    text::Text,
    widgets::{Block, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.size());

    match app.current_screen {
        CurrentScreen::EpicsList => {
            let epics = &app.current_epics.data;
            let mut list_items: Vec<ListItem> = vec![];
            for epic in epics.into_iter() {
                list_items.push(ListItem::new(epic.name.to_string()))
            }
            let body_block = Block::default().style(Style::default());
            let mut state = ListState::default().with_selected(Some(app.selected_index));
            let body = List::new(list_items)
                .block(body_block)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::Magenta));
            frame.render_stateful_widget(body, layout[0], &mut state);

            let footer_block = Block::default().style(Style::default());
            let footer = Paragraph::new(Text::styled(
                format!(
                    "cutty 0.1.0 | @{} | [q]uit | [r]efresh | [s]tories list | [Enter] Show stories in epic",
                    app.current_member.mention_name
                ),
                Style::default().fg(Color::LightBlue),
            ))
            .block(footer_block);
            frame.render_widget(footer, layout[1]);
        }
        CurrentScreen::StoriesList => {
            let stories = &app.current_stories.data;
            let mut list_items: Vec<ListItem> = vec![];
            for story in stories.into_iter() {
                list_items.push(ListItem::new(story.name.to_string()))
            }
            let body_block = Block::default().style(Style::default());
            let mut state = ListState::default().with_selected(Some(app.selected_index));
            let body = List::new(list_items)
                .block(body_block)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::Magenta));
            frame.render_stateful_widget(body, layout[0], &mut state);

            let footer_block = Block::default().style(Style::default());
            let footer = Paragraph::new(Text::styled(
                format!(
                    "cutty 0.1.0 | @{} | [q]uit | [r]efresh | [e]pics list | [Enter] Open story",
                    app.current_member.mention_name
                ),
                Style::default().fg(Color::LightBlue),
            ))
            .block(footer_block);
            frame.render_widget(footer, layout[1]);
        }
        CurrentScreen::StoryDetail => {
            let story = &app.current_stories.data[app.selected_index];
            let workflow = &app
                .workflows
                .iter()
                .find(|workflow| workflow.id == story.workflow_id)
                .unwrap();
            let workflow_state = &workflow
                .states
                .iter()
                .find(|state| state.id == story.workflow_state_id)
                .unwrap();

            let mut body_lines = vec![
                Line::from(story.name.to_string()).style(Style::default().fg(Color::LightBlue)),
                Line::from(story.app_url.to_string()).style(Style::default().fg(Color::LightGreen)),
                Line::from(workflow.name.to_string()).style(Style::default().fg(Color::Magenta)),
                Line::from(workflow_state.name.to_string())
                    .style(Style::default().fg(Color::LightRed)),
            ];
            for line in story.description.split("\n").into_iter() {
                body_lines.push(Line::from(line.to_string()));
            }
            for comment in &story.comments {
                body_lines.push(Line::from(comment.created_at.to_string()));
                body_lines.push(Line::from(comment.author.as_ref().unwrap().profile.mention_name.to_string()));
                match &comment.text {
                    Some(text) => {
                        for line in text.split("\n").into_iter() {
                            body_lines.push(Line::from(line.to_string()));
                        }
                    }
                    None => ()
                }
                body_lines.push(Line::from("-----------------------------"));
            }
            let body_block = Block::default().style(Style::default());
            let body = Paragraph::new(body_lines)
                .wrap(Wrap { trim: false })
                .scroll((app.scroll, 0))
                .block(body_block);
            frame.render_widget(body, layout[0]);

            let footer_block = Block::default().style(Style::default());
            let footer = Paragraph::new(Text::styled(
                format!(
                    "cutty 0.1.0 | @{} | [b]ack | update [s]tate",
                    app.current_member.mention_name
                ),
                Style::default().fg(Color::LightBlue),
            ))
            .block(footer_block);
            frame.render_widget(footer, layout[1]);
        }
    }
}
