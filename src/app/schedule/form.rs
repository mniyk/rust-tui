use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Layout}, 
    Frame,
};

use crate::app::ui::{
    pane::Pane,
    popup::Popup,
    input::Input,
};

const FORM_TITLE: &str = "Add Schedule";
const SUMMARY_TITLE: &str = "Summary";
const START_TITLE: &str = "Start";
const END_TITLE: &str = "End";
const DESCRIPTION_TITLE: &str = "Description";

#[derive(Debug)]
pub struct Form {
    pub popup: Popup,
    pub summary_pane: Pane,
    pub summary: Input,
    pub start_pane: Pane,
    pub start: Input,
    pub end_pane: Pane,
    pub end: Input,
    pub description_pane: Pane,
    pub description: Input,
}

impl Form {
    pub fn new() -> Self {
        Self {
            popup: Popup::new(FORM_TITLE),
            summary_pane: Pane::new(SUMMARY_TITLE),
            summary: Input::new(),
            start_pane: Pane::new(START_TITLE),
            start: Input::new(),
            end_pane: Pane::new(END_TITLE),
            end: Input::new(),
            description_pane: Pane::new(DESCRIPTION_TITLE),
            description: Input::new(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        if self.popup.active {
            let popup_area = self.popup.render(frame, [80, 80]);

            let horizontal = Layout::horizontal([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ]);
            let [_, form_area, _] = horizontal.areas(popup_area);

            let layout = Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ]);
            let [_, summary_area, start_area, end_area, description_area, _] = 
                layout.areas(form_area);

            let summary_pane = self.summary_pane.render(frame, summary_area);
            self.summary.render(frame, summary_pane);

            let start_pane = self.start_pane.render(frame, start_area);
            self.start.render(frame, start_pane);

            let end_pane = self.end_pane.render(frame, end_area);
            self.end.render(frame, end_pane);

            let description_pane = self.description_pane.render(frame, description_area);
            self.description.render(frame, description_pane);
        }
    }

    pub fn all_clear(&mut self) {
        self.summary.clear();
        self.start.clear();
        self.end.clear();
        self.description.clear();
    }

    pub fn active_summary(&mut self) {
        self.summary_pane.active = true;
        self.summary.active = true;
        self.start_pane.active = false;
        self.start.active = false;
        self.end_pane.active = false;
        self.end.active = false;
        self.description_pane.active = false;
        self.description.active = false;
    }

    pub fn active_start(&mut self) {
        self.summary_pane.active = false;
        self.summary.active = false;
        self.start_pane.active = true;
        self.start.active = true;
        self.end_pane.active = false;
        self.end.active = false;
        self.description_pane.active = false;
        self.description.active = false;
    }

    pub fn active_end(&mut self) {
        self.summary_pane.active = false;
        self.summary.active = false;
        self.start_pane.active = false;
        self.start.active = false;
        self.end_pane.active = true;
        self.end.active = true;
        self.description_pane.active = false;
        self.description.active = false;
    }

    pub fn active_description(&mut self) {
        self.summary_pane.active = false;
        self.summary.active = false;
        self.start_pane.active = false;
        self.start.active = false;
        self.end_pane.active = false;
        self.end.active = false;
        self.description_pane.active = true;
        self.description.active = true;
    }

    pub fn key_binding(&mut self, key: KeyEvent) {
        if self.summary.active {
            self.summary.key_binding(key);
        } else if self.start.active {
            self.start.key_binding(key);
        } else if self.end.active {
            self.end.key_binding(key);
        } else if self.description.active {
            self.description.key_binding(key);
        }
    }
}