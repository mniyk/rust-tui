use ratatui::crossterm::event::KeyCode;

use crate::app::google::calendar::calendar::{
    Schedules,
    Mode,
    Form,
};
use crate::app::google::authentication::TokenInfo;
use crate::app::TabMode;

pub struct KeyBind {}

impl KeyBind {
    pub fn execute(token_info: &TokenInfo, key_code: KeyCode, selected_tab: &mut TabMode, schedules: &mut Schedules) {
        match key_code {
            KeyCode::Up => Self::up(schedules),
            KeyCode::Down => Self::down(schedules),
            KeyCode::Right => Self::right(schedules),
            KeyCode::Left => Self::left(schedules),
            KeyCode::Enter => Self::enter(token_info, schedules),
            KeyCode::Esc => Self::esc(schedules),
            KeyCode::Tab => Self::tab(selected_tab, schedules),
            KeyCode::Backspace => Self::backspace(schedules),
            KeyCode::Char('N') => Self::char_n(schedules),
            KeyCode::Char('C') => Self::char_c(token_info, schedules),
            KeyCode::Char(insert_char) => Self::char(insert_char, schedules),
            _ => {}
        }
    }
    
    fn up(schedules: &mut Schedules) {
        match schedules.mode {
            Mode::List => {
                if schedules.selected_index > 0 {
                    schedules.selected_index -= 1;
                }
            }
            _ => {}
        }
    }

    fn down(schedules: &mut Schedules) {
        match schedules.mode {
            Mode::List => {
                if schedules.schedules.len() > 0 {
                    if schedules.selected_index < schedules.schedules.len() - 1 {
                        schedules.selected_index += 1;
                    }
                }
            }
            _ => {}
        }
    }

    fn right(schedules: &mut Schedules) {
        match schedules.mode {
            Mode::List => {},
            Mode::New => match schedules.selected_form {
                Form::Summary => {
                    let cursor_move_right = schedules.summary.index.saturating_add(1);
                    schedules.summary.index = cursor_move_right.clamp(0, schedules.summary.text.chars().count());
                }
                Form::Start => {
                    let cursor_move_right = schedules.start.index.saturating_add(1);
                    schedules.start.index = cursor_move_right.clamp(0, schedules.start.text.chars().count());
                }
                Form::End => {
                    let cursor_move_right = schedules.end.index.saturating_add(1);
                    schedules.end.index = cursor_move_right.clamp(0, schedules.end.text.chars().count());
                }
                Form::Description => {
                    let cursor_move_right = schedules.description.index.saturating_add(1);
                    schedules.description.index = cursor_move_right.clamp(0, schedules.description.text.chars().count());
                }
            }
        }
    }

    fn left(schedules: &mut Schedules) {
        match schedules.mode {
            Mode::List => {}
            Mode::New => match schedules.selected_form {
                Form::Summary => {
                    let cursor_move_left = schedules.summary.index.saturating_sub(1);
                    schedules.summary.index = cursor_move_left.clamp(0, schedules.summary.text.chars().count());
                }
                Form::Start => {
                    let cursor_move_left = schedules.start.index.saturating_sub(1);
                    schedules.start.index = cursor_move_left.clamp(0, schedules.start.text.chars().count());
                }
                Form::End => {
                    let cursor_move_left = schedules.end.index.saturating_sub(1);
                    schedules.end.index = cursor_move_left.clamp(0, schedules.end.text.chars().count());
                }
                Form::Description => {
                    let cursor_move_left = schedules.description.index.saturating_sub(1);
                    schedules.description.index = cursor_move_left.clamp(0, schedules.description.text.chars().count());
                }
            }
        }
    }

    fn enter(token_info: &TokenInfo, schedules: &mut Schedules) {
        match schedules.mode {
            Mode::List => schedules.open_schedule(),
            Mode::New => {
                schedules.write_schedule(&token_info);
                *schedules = Schedules::new(token_info);
            }
        }
    }
    
    fn esc(schedules: &mut Schedules) {
        match schedules.mode {
            Mode::List => {},
            Mode::New => {
                schedules.mode = Mode::List;
                schedules.selected_form = Form::Summary;
                schedules.summary.index = 0;
                schedules.summary.text = "".to_string();
                schedules.start.index = 0;
                schedules.start.text = "".to_string();
                schedules.end.index = 0;
                schedules.end.text = "".to_string();
                schedules.description.index = 0;
                schedules.description.text = "".to_string();
            },
        }
    }
    
    fn tab(selected_tab: &mut TabMode, schedules: &mut Schedules) {
        match schedules.mode {
            Mode::List => *selected_tab = TabMode::VirtualBox,
            Mode::New => match schedules.selected_form {
                Form::Summary => schedules.selected_form = Form::Start,
                Form::Start => schedules.selected_form = Form::End,
                Form::End => schedules.selected_form = Form::Description,
                Form::Description => schedules.selected_form = Form::Summary,
            }
        }
    }
    
    fn backspace(schedules: &mut Schedules) {
        match schedules.mode {
            Mode::List => {},
            Mode::New => match schedules.selected_form {
                Form::Summary => {
                    if schedules.summary.index != 0 {
                        let current_index = schedules.summary.index;
                        let from_left_to_current_index = current_index - 1;
                        let before_char_to_delete = schedules.summary.text.chars().take(from_left_to_current_index);
                        let after_char_to_delete = schedules.summary.text.chars().skip(current_index);
                        
                        schedules.summary.text = before_char_to_delete.chain(after_char_to_delete).collect();
                        let cursor_move_left = schedules.summary.index.saturating_sub(1);
                        schedules.summary.index = cursor_move_left.clamp(0, schedules.summary.text.chars().count());
                    }
                }
                Form::Start => {
                    if schedules.start.index != 0 {
                        let current_index = schedules.start.index;
                        let from_left_to_current_index = current_index - 1;
                        let before_char_to_delete = schedules.start.text.chars().take(from_left_to_current_index);
                        let after_char_to_delete = schedules.start.text.chars().skip(current_index);
                        
                        schedules.start.text = before_char_to_delete.chain(after_char_to_delete).collect();
                        let cursor_move_left = schedules.start.index.saturating_sub(1);
                        schedules.start.index = cursor_move_left.clamp(0, schedules.start.text.chars().count());
                    }
                }
                Form::End => {
                    if schedules.end.index != 0 {
                        let current_index = schedules.end.index;
                        let from_left_to_current_index = current_index - 1;
                        let before_char_to_delete = schedules.end.text.chars().take(from_left_to_current_index);
                        let after_char_to_delete = schedules.end.text.chars().skip(current_index);
                        
                        schedules.end.text = before_char_to_delete.chain(after_char_to_delete).collect();
                        let cursor_move_left = schedules.start.index.saturating_sub(1);
                        schedules.end.index = cursor_move_left.clamp(0, schedules.end.text.chars().count());
                    }
                }
                Form::Description => {
                    if schedules.description.index != 0 {
                        let current_index = schedules.description.index;
                        let from_left_to_current_index = current_index - 1;
                        let before_char_to_delete = schedules.description.text.chars().take(from_left_to_current_index);
                        let after_char_to_delete = schedules.description.text.chars().skip(current_index);
                        
                        schedules.description.text = before_char_to_delete.chain(after_char_to_delete).collect();
                        let cursor_move_left = schedules.start.index.saturating_sub(1);
                        schedules.description.index = cursor_move_left.clamp(0, schedules.description.text.chars().count());
                    }
                }
            }
        }
    }
    
    fn char_n(schedules: &mut Schedules) {
        match schedules.mode {
            Mode::List => schedules.mode = Mode::New,
            _ => {},
        }
    }
    
    fn char_c(token_info: &TokenInfo, schedules: &mut Schedules) {
        match schedules.mode {
            Mode::List => {
                schedules.delete_schedule(token_info);
                *schedules = Schedules::new(token_info);
            }
            _ => {}
        }
    }

    fn char(insert_char: char, schedules: &mut Schedules) {
        match schedules.mode {
            Mode::List => {}
            Mode::New => match schedules.selected_form {
                Form::Summary => {
                    let index = schedules.summary.text
                        .char_indices()
                        .map(|(i, _)| i)
                        .nth(schedules.summary.index)
                        .unwrap_or(schedules.summary.text.len());
                    schedules.summary.text.insert(index, insert_char);
                    let cursor_move_right = schedules.summary.index.saturating_add(1);
                    schedules.summary.index = cursor_move_right.clamp(0, schedules.summary.text.chars().count());
                }
                Form::Start => {
                    let index = schedules.start.text
                        .char_indices()
                        .map(|(i, _)| i)
                        .nth(schedules.start.index)
                        .unwrap_or(schedules.start.text.len());
                    schedules.start.text.insert(index, insert_char);
                    let cursor_move_right = schedules.start.index.saturating_add(1);
                    schedules.start.index = cursor_move_right.clamp(0, schedules.start.text.chars().count());
                }
                Form::End => {
                    let index = schedules.end.text
                        .char_indices()
                        .map(|(i, _)| i)
                        .nth(schedules.end.index)
                        .unwrap_or(schedules.end.text.len());
                    schedules.end.text.insert(index, insert_char);
                    let cursor_move_right = schedules.end.index.saturating_add(1);
                    schedules.end.index = cursor_move_right.clamp(0, schedules.end.text.chars().count());
                }
                Form::Description => {
                    let index = schedules.description.text
                        .char_indices()
                        .map(|(i, _)| i)
                        .nth(schedules.description.index)
                        .unwrap_or(schedules.description.text.len());
                    schedules.description.text.insert(index, insert_char);
                    let cursor_move_right = schedules.description.index.saturating_add(1);
                    schedules.description.index = cursor_move_right.clamp(0, schedules.description.text.chars().count());
                }
            }
        }
    }
}