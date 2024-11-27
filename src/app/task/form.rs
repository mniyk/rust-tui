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

const TITLE_TITLE: &str = "Title";
const NOTES_TITLE: &str = "Notes";
const DUE_TITLE: &str = "Due";

#[derive(Debug)]
pub enum Mode {
    New,
    Edit,
}

#[derive(Debug)]
pub struct Form {
    pub mode: Mode,
    pub popup: Popup,
    pub title_pane: Pane,
    pub title: Input,
    pub notes_pane: Pane,
    pub notes: Input,
    pub due_pane: Pane,
    pub due: Input,
}

impl Form {
    pub fn new() -> Self {
        Self {
            mode: Mode::New,
            popup: Popup::new(""),
            title_pane: Pane::new(TITLE_TITLE),
            title: Input::new(),
            notes_pane: Pane::new(NOTES_TITLE),
            notes: Input::new(),
            due_pane: Pane::new(DUE_TITLE),
            due: Input::new(),
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
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Length(1),
            ]);
            let [_, title_area, notes_area, due_area,  _] = layout.areas(form_area);

            let title_pane = self.title_pane.render(frame, title_area);
            self.title.render(frame, title_pane);

            let notes_pane = self.notes_pane.render(frame, notes_area);
            self.notes.render(frame, notes_pane);

            let due_pane = self.due_pane.render(frame, due_area);
            self.due.render(frame, due_pane);
        }
    }

    pub fn all_clear(&mut self) {
        self.title.clear();
        self.notes.clear();
        self.due.clear();
    }

    pub fn active_title(&mut self) {
        self.title_pane.active = true;
        self.title.active = true;
        self.notes_pane.active = false;
        self.notes.active = false;
        self.due_pane.active = false;
        self.due.active = false;
    }

    pub fn active_notes(&mut self) {
        self.title_pane.active = false;
        self.title.active = false;
        self.notes_pane.active = true;
        self.notes.active = true;
        self.due_pane.active = false;
        self.due.active = false;
    }

    pub fn active_due(&mut self) {
        self.title_pane.active = false;
        self.title.active = false;
        self.notes_pane.active = false;
        self.notes.active = false;
        self.due_pane.active = true;
        self.due.active = true;
    }

    pub fn key_binding(&mut self, key: KeyEvent) {
        if self.title.active {
            self.title.key_binding(key);
        } else if self.notes.active {
            self.notes.key_binding(key);
        } else if self.due.active {
            self.due.key_binding(key);
        }
    }
}