use std::{fs, process::Command};

use serde::Deserialize;

const JSON_PATH: &str = "bookmark.json";
const BROWSER_PATH: &str = "/mnt/c/Program Files/BraveSoftware/Brave-Browser/Application/brave.exe";

#[derive(Debug, Deserialize)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
}

#[derive(Debug)]
pub struct Bookmarks {
    pub selected_index: usize,
    pub bookmarks: Vec<Bookmark>,
}

impl Bookmarks {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            bookmarks: Self::read_bookmark(),
        }
    }

    fn read_bookmark() -> Vec<Bookmark> {
        let data = fs::read_to_string(JSON_PATH);

        match data {
            Ok(d) => serde_json::from_str(&d).unwrap_or_else(|_| Vec::new()),
            Err(_) => Vec::new(),
        }
    }

    pub fn open_browser(&self) {
        let bookmark = self.bookmarks[self.selected_index].url.to_string();

        Command::new(BROWSER_PATH)
            .arg(bookmark)
            .output()
            .expect("Failed to execute open_browser");
    }
}