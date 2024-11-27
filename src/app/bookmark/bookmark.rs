use std::{fs, process::Command};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect, 
    Frame,
};
use serde::Deserialize;

use crate::app::ui::{
    help::Help,
    select_list::SelectList,
    pane::Pane,
};

const JSON_PATH: &str = "bookmark.json";
const BROWSER_PATH: &str = "/mnt/c/Program Files/BraveSoftware/Brave-Browser/Application/brave.exe";
const APP_TITLE: &str = "Bookmark";
const HELP_TITLE: &str = "Help Bookmark";

#[derive(Debug, Deserialize)]
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
    pub select_list: SelectList<'a>,
    pub help: Help,
}

impl<'a> Bookmarks<'a> {
    pub fn new() -> Self {
        Self {
            pane: Pane::new(APP_TITLE),
            bookmarks: Self::read(),
            select_list: SelectList::new(),
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

        self.select_list.render(frame, pane, list);
    }

    pub fn open(&self) {
        let bookmark = self.bookmarks[self.select_list.index].url.to_string();

        Command::new(BROWSER_PATH)
            .arg(bookmark)
            .output()
            .expect("Failed to execute open()");
    }

    pub fn key_binding(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::F(1) => self.help.popup.active = true,
            KeyCode::Enter => self.open(),
            _ => self.select_list.key_binding(key),
        }
    }
}