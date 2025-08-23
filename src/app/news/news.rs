use std::process::Command;

use reqwest;
use roxmltree::Document;
use chrono::DateTime;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect, 
    Frame
};
use serde::Serialize;
use tokio;

use crate::app::config::get_config;
use crate::app::ui::{
    help::Help,
    select_list::SelectList,
    pane::Pane,
};

const APP_TITLE: &str = "News";
const HELP_TITLE: &str = "Help News";

#[derive(Debug, Serialize)]
pub struct News {
    pub title: String,
    pub pub_date: String,
    pub link: String,
}

impl News {
    fn value(&self) -> String {
        format!("{}\n  {}", self.pub_date, self.title)
    }
}

#[derive(Debug)]
pub struct NewsItems<'a> {
    pub pane: Pane,
    pub news_items: Vec<News>,
    pub select_list: SelectList<'a>,
    pub help: Help,
}

impl<'a> NewsItems<'a> {
    pub fn new() -> Self {
        Self {
            pane: Pane::new(APP_TITLE),
            news_items: Self::read(),
            select_list: SelectList::new(),
            help: Help::new(HELP_TITLE),
        }
    }

    fn read() -> Vec<News> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {

            let url = "https://news.google.com/rss?hl=ja&gl=JP&ceid=JP:ja";
            
            let client = reqwest::Client::new();
            let response = match client.get(url).send().await {
                Ok(resp) => resp,
                Err(_) => return Vec::new(),
            };
            
            let xml_content = match response.text().await {
                Ok(text) => text,
                Err(_) => return Vec::new(),
            };
            
            let doc = match Document::parse(&xml_content) {
                Ok(document) => document,
                Err(_) => return Vec::new(),
            };
            
            let mut news = Vec::new();

            for row in doc.descendants().filter(|n| n.tag_name().name() == "item") {
                let mut title = String::new();
                let mut pub_date = String::new();
                let mut link = String::new();

                for child in row.children().filter(|n| n.is_element()) {
                    match child.tag_name().name() {
                        "title" => {
                            if let Some(text) = child.text() {
                                title = text.to_string();
                            }
                        }
                        "pubDate" => {
                            if let Some(text) = child.text() {
                                pub_date = text.to_string();
                            }
                        }
                        "link" => {
                            if let Some(text) = child.text() {
                                link = text.to_string();
                            }
                        }
                        _ => {}
                    }
                }

                if !title.is_empty() && !pub_date.is_empty() && !link.is_empty() {
                    news.push(News {
                        title,
                        pub_date,
                        link,
                    });
                }
            }

            news.sort_by(|a, b| {
                let date_a = DateTime::parse_from_rfc2822(&a.pub_date);
                let date_b = DateTime::parse_from_rfc2822(&b.pub_date);
                
                match (date_a, date_b) {
                    (Ok(a), Ok(b)) => b.cmp(&a),
                    (Ok(_), Err(_)) => std::cmp::Ordering::Less,
                    (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
                    (Err(_), Err(_)) => std::cmp::Ordering::Equal,
                }
            });

            news
        })
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let pane = self.pane.render(frame, area);

        let list = self.news_items.iter().map(|news| {
            news.value().clone()
        })
        .collect();

        self.select_list.render(frame, pane, list);
    }

    pub fn open(&self) {
        if self.news_items.is_empty() {
            return;
        }
        let news: String = self.news_items[self.select_list.index].link.to_string();

        let browser_path = &get_config().browser_path;

        Command::new(browser_path)
            .arg(news)
            .output()
            .expect("Failed to execute open_news()");
    }

    pub fn key_binding(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::F(1) => {
                if !self.help.popup.active {
                    self.help.popup.active = true;
                } else {
                    self.help.popup.active = false;
                }
            },
            KeyCode::Enter => {
                if !self.help.popup.active {
                    self.open();
                }
            },
            KeyCode::Char('R') => {
                self.news_items = Self::read();
            },
            _ => {
                if !self.help.popup.active {
                    self.select_list.key_binding(key);
                }
            }
        }
    }
}
