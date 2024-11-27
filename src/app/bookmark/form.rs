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
const URL_TITLE: &str = "URL";

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
    pub url_pane: Pane,
    pub url: Input,
}

impl Form {
    pub fn new() -> Self {
        Self {
            mode: Mode::New,
            popup: Popup::new(""),
            title_pane: Pane::new(TITLE_TITLE),
            title: Input::new(),
            url_pane: Pane::new(URL_TITLE),
            url: Input::new(),
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
                Constraint::Length(1),
            ]);
            let [_, title_area, url_area, _] = 
                layout.areas(form_area);

            let title_pane = self.title_pane.render(frame, title_area);
            self.title.render(frame, title_pane);

            let url_pane = self.url_pane.render(frame, url_area);
            self.url.render(frame, url_pane);
        }
    }

    pub fn all_clear(&mut self) {
        self.title.clear();
        self.url.clear();
    }

    pub fn active_title(&mut self) {
        self.title_pane.active = true;
        self.title.active = true;
        self.url_pane.active = false;
        self.url.active = false;
    }

    pub fn active_url(&mut self) {
        self.title_pane.active = false;
        self.title.active = false;
        self.url_pane.active = true;
        self.url.active = true;
    }

    pub fn key_binding(&mut self, key: KeyEvent) {
        if self.title.active {
            self.title.key_binding(key);
        } else if self.url.active {
            self.url.key_binding(key);
        }
    }
}