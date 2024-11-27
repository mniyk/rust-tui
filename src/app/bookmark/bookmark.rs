use std::{fs, process::Command};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent}, 
    layout::Rect, 
    Frame
};
use serde::{Deserialize, Serialize};

use crate::app::ui::{
    help::Help,
    select_list::SelectList,
    pane::Pane,
};

use super::form::{Form, Mode as FormMode};

const JSON_PATH: &str = "bookmark.json";
const BROWSER_PATH: &str = "/mnt/c/Program Files/BraveSoftware/Brave-Browser/Application/brave.exe";
const APP_TITLE: &str = "Bookmark";
const HELP_TITLE: &str = "Help Bookmark";

#[derive(Debug, Deserialize, Serialize)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
}

impl Bookmark {
    fn value(&self) -> String {
        format!("{}", self.title)
    }
}

#[derive(Debug)]
pub struct Bookmarks<'a> {
    pub pane: Pane,
    pub bookmarks: Vec<Bookmark>,
    pub list: SelectList<'a>,
    pub form: Form,
    pub help: Help,
}

impl<'a> Bookmarks<'a> {
    pub fn new() -> Self {
        Self {
            pane: Pane::new(APP_TITLE),
            bookmarks: Self::read(),
            list: SelectList::new(),
            form: Form::new(),
            help: Help::new(HELP_TITLE)
        }
    }

    fn read() -> Vec<Bookmark> {
        let data = fs::read_to_string(JSON_PATH);

        match data {
            Ok(d) => serde_json::from_str(&d).unwrap_or_else(|_| Vec::new()),
            Err(_) => Vec::new(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let pane = self.pane.render(frame, area);

        let list = self.bookmarks.iter().map(|bookmark| {
            bookmark.value().clone()
        })
        .collect();

        self.list.render(frame, pane, list);
    }

    pub fn open(&self) {
        let bookmark = self.bookmarks[self.list.index].url.to_string();

        Command::new(BROWSER_PATH)
            .arg(bookmark)
            .output()
            .expect("Failed to execute open()");
    }

    pub fn add(&mut self) {
        let bookmark = Bookmark {
            title: self.form.title.text.to_string(),
            url: self.form.url.text.to_string(),
        };

        self.bookmarks.push(bookmark);

        self.output();

        self.form.popup.active = false;
        self.form.all_clear();
    }

    pub fn edit(&mut self) {
        let bookmark = &mut self.bookmarks[self.list.index];

        bookmark.title = self.form.title.text.to_string();
        bookmark.url = self.form.url.text.to_string();

        self.output();

        self.form.popup.active = false;
        self.form.all_clear();
    }

    pub fn delete(&mut self) {
        _ = self.bookmarks.remove(self.list.index);

        self.output();
    }

    fn output(&self) {
        let json_string = serde_json::to_string_pretty(&self.bookmarks)
            .expect("Failed to execute delete()");

        fs::write(JSON_PATH, json_string).expect("Failed to execute output()");
    }

    pub fn key_binding(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::F(1) => {
                if !self.help.popup.active & !self.form.popup.active {
                    self.help.popup.active = true;
                    self.form.popup.active = false;
                }
            }
            KeyCode::F(2) => {
                if !self.help.popup.active & !self.form.popup.active {
                    self.form.popup.title = "Add Bookmark".to_string();
                    self.form.mode = FormMode::New;
                    self.help.popup.active = false;
                    self.form.popup.active = true;
                    self.form.all_clear();
                    self.form.active_title();
                }
            }
            KeyCode::F(3) => {
                if !self.help.popup.active & !self.form.popup.active {
                    self.form.popup.title = "Edit Bookmark".to_string();
                    self.form.mode = FormMode::Edit;
                    self.help.popup.active = false;
                    self.form.popup.active = true;
                    self.form.all_clear();
                    self.form.active_title();

                    let bookmark = &self.bookmarks[self.list.index];
                    self.form.title.text = bookmark.title.to_string();
                    self.form.url.text = bookmark.url.to_string();
                }
            }
            KeyCode::F(12) => {
                if self.form.popup.active {
                    match self.form.mode {
                        FormMode::New => self.add(),
                        FormMode::Edit => self.edit(),
                    }
                }
            }
            KeyCode::Enter => {
                if !self.help.popup.active & !self.form.popup.active {
                    self.open();
                }
            },
            KeyCode::Char('D') => self.delete(),
            _ => {
                if self.form.popup.active {
                    self.form.key_binding(key);
                } else if !self.help.popup.active {
                    self.list.key_binding(key);
                }
            }
        }
    }
}