use ratatui::{
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::google::calendar::calendar::{
    Schedules,
    Mode,
    Form,
};
use crate::app::WindowMode;

pub struct UI {}

impl UI {
    pub fn schedule_area(frame: &mut Frame, area: Rect, window_mode: &WindowMode, schedules: &Schedules) {
        let schedule_list_item = 
            schedules.schedules.iter().enumerate().map(|(i, schedule)| {
                if i == schedules.selected_index {
                    ListItem::new(
                        format!(
                            "> {}\n  {} - {}", 
                            schedule.summary.to_string(),
                            schedule.start.to_string(),
                            schedule.end.to_string(),
                        )
                    ).style(Style::default().fg(Color::Green))
                } else {
                    ListItem::new(
                        format!(
                            "{}\n  {} - {}", 
                            schedule.summary.to_string(),
                            schedule.start.to_string(),
                            schedule.end.to_string(),
                        )
                    )
                }
            });
        
        let schedule_list = match window_mode {
            WindowMode::Tab => {
                List::new(schedule_list_item)
                    .block(
                        Block::default()
                            .title("  Schedule  ")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Green))
                    )
            }
            _ => {
                List::new(schedule_list_item)
                    .block(
                        Block::default()
                            .title("  Schedule  ")
                            .borders(Borders::ALL)
                    )
            }
        };
        frame.render_widget(schedule_list, area);

        match schedules.mode {
            Mode::New => {
                Self::input_area(frame, schedules);
            }
            _ => {},
        }
    }

    fn input_area(frame: &mut Frame, schedules: &Schedules) {
        let vertical = Layout::vertical([Constraint::Percentage(43)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(80)]).flex(Flex::Center);
        let [area] = vertical.areas(frame.area());
        let [area] = horizontal.areas(area);

        let block = Block::bordered().border_style(Style::default().fg(Color::Green)).title("  New Schedule  ");

        frame.render_widget(Clear, area);
        frame.render_widget(block, area);

        let input_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
        ]).margin(2);
        let [summary_area, start_area, end_area, description_area, footer_area] = 
            input_layout.areas(area);

        Self::summary_input(frame, summary_area, schedules);
        Self::start_input(frame, start_area, schedules);
        Self::end_input(frame, end_area, schedules);
        Self::description_input(frame, description_area, schedules);
        Self::footer_area(frame, footer_area);
    }

    fn summary_input(frame: &mut Frame, area: Rect, schedules: &Schedules) {
        let summary_input = match schedules.selected_form {
            Form::Summary => {
                frame.set_cursor_position(Position::new(
                    area.x + schedules.summary.index as u16 + 1,
                    area.y + 1
                ));
                Paragraph::new(schedules.summary.text.clone())
                    .block(
                        Block::bordered()
                            .title("  Summary  ")
                            .border_style(Style::default().fg(Color::Green))
                    )
            }
            _ => Paragraph::new(schedules.summary.text.clone()).block(Block::bordered().title("  Summary  ")),
        };
        frame.render_widget(summary_input, area);
    }

    fn start_input(frame: &mut Frame, area: Rect, schedules: &Schedules) {
        let start_input = match schedules.selected_form {
            Form::Start => {
                frame.set_cursor_position(Position::new(
                    area.x + schedules.start.index as u16 + 1,
                    area.y + 1
                ));
                Paragraph::new(schedules.start.text.clone())
                    .block(
                        Block::bordered()
                            .title("  Start  ")
                            .border_style(Style::default().fg(Color::Green))
                    )
            }
            _ => Paragraph::new(schedules.start.text.clone()).block(Block::bordered().title("  Start  "))
        };
        frame.render_widget(start_input, area);
    }

    fn end_input(frame: &mut Frame, area: Rect, schedules: &Schedules) {
        let end_input = match schedules.selected_form {
            Form::End => {
                frame.set_cursor_position(Position::new(
                    area.x + schedules.end.index as u16 + 1,
                    area.y + 1
                ));
                Paragraph::new(schedules.end.text.clone())
                    .block(
                        Block::bordered()
                            .title("  End  ")
                            .border_style(Style::default().fg(Color::Green))
                    )
            }
            _ => Paragraph::new(schedules.end.text.clone()).block(Block::bordered().title("  End  "))
        };
        frame.render_widget(end_input, area);
    }

    fn description_input(frame: &mut Frame, area: Rect, schedules: &Schedules) {
        let description_input = match schedules.selected_form {
            Form::Description => {
                frame.set_cursor_position(Position::new(
                    area.x + schedules.description.index as u16 + 1,
                    area.y + 1
                ));
                Paragraph::new(schedules.description.text.clone())
                    .block(
                        Block::bordered()
                            .title("  Description  ")
                            .border_style(Style::default().fg(Color::Green))
                    )
            }
            _ => Paragraph::new(schedules.description.text.clone()).block(Block::bordered().title("  Description  "))
        };
        frame.render_widget(description_input, area);
    }

    fn footer_area(frame: &mut Frame, area: Rect) {
        let line1 = Line::from(vec![
            Span::raw("  Esc: Cancel, Enter: Create"),
        ]);
        let footer_text = Paragraph::new(
            vec![line1]
        );
        frame.render_widget(footer_text, area);
    }
}