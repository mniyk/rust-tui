use std::process::Command;

use reqwest::blocking::Client;
use chrono::{Utc, FixedOffset, TimeZone};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect, 
    Frame,
};
use serde_json::json;

use crate::app::schedule::form::{Form, Mode as FormMode};
use crate::app::ui::{
    help::Help,
    select_list::SelectList,
    pane::Pane,
};
use crate::app::google::authentication::TokenInfo;

const BASE_API: &str = "https://www.googleapis.com/calendar/v3/calendars/primary/events";
const BROWSER_PATH: &str = "/mnt/c/Program Files/BraveSoftware/Brave-Browser/Application/brave.exe";
const APP_TITLE: &str = "Schedule";
const HELP_TITLE: &str = "Help Schedule";

#[derive(Debug)]
pub struct Schedule {
    pub id: String,
    pub summary: String,
    pub start: String, 
    pub end: String,
    pub link: String,
    pub description: String,
}

impl Schedule {
    fn value(&self) -> String {
        format!("{}\n  {} - {}", self.summary, self.start, self.end)
    }
}

#[derive(Debug)]
pub struct Schedules<'a> {
    pub pane: Pane,
    pub schedules: Vec<Schedule>,
    pub list: SelectList<'a>,
    pub form: Form,
    pub help: Help,
}

impl<'a> Schedules<'a> {
    pub fn new(token_info: &TokenInfo) -> Self {
        Self {
            pane: Pane::new(APP_TITLE),
            schedules: Self::read(token_info),
            list: SelectList::new(),
            form: Form::new() ,
            help: Help::new(HELP_TITLE),
        }
    }

    pub fn read(token_info: &TokenInfo) -> Vec<Schedule> {
        let client = Client::new();
        let [time_min, time_max] = Self::time_min_max();

        let response = client.get(BASE_API)
            .bearer_auth(&token_info.access_token)
            .query(&[
                ("orderBy", "startTime"),
                ("singleEvents", "true"),
                ("timeMin", &time_min),
                ("timeMax", &time_max),
            ])
            .send()
            .unwrap();

        let mut schedules = Vec::<Schedule>::new();
        if response.status().is_success() {
            let events: serde_json::Value = response.json().unwrap();
            if let Some(items) = events["items"].as_array() {
                if !items.is_empty() {
                    for item in items {
                        let id = item["id"].to_string().replace("\"", "");
                        let summary = item["summary"].as_str().unwrap_or("No summary");
                        let description = item["description"].as_str().unwrap_or("No discription");
                        let start = item["start"]["dateTime"]
                            .as_str()
                            .or_else(|| item["start"]["date"].as_str())
                            .unwrap_or("No start time");
                        let end = item["end"]["dateTime"]
                            .as_str()
                            .or_else(|| item["end"]["date"].as_str())
                            .unwrap_or("No end time");
                        let link = item["htmlLink"].to_string().replace("\"", "");

                        let schedule = Schedule {
                            id: id.to_string(),
                            summary: summary.to_string(),
                            start: start.to_string(),
                            end: end.to_string(),
                            link,
                            description: description.to_string(),
                        };

                        schedules.push(schedule);
                    }
                }
            }
        };

        schedules
    }

    pub fn time_min_max() -> [String; 2] {
        let jst = FixedOffset::east_opt(9 * 3600).expect("Invalid offset");
        let now = Utc::now().with_timezone(&jst);
        
        let time_min_naive = now
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .expect("Failed to create time_min");
        let time_min_result = jst
            .from_local_datetime(&time_min_naive);
        let time_min = match time_min_result {
            chrono::LocalResult::Single(dt) => dt.to_rfc3339(),
            _ => return ["".to_string(), "".to_string()],
        };

        let time_max_naive = (now + chrono::Duration::days(1))
            .date_naive()
            .and_hms_opt(23, 59, 59)
            .expect("Failed to create time_max");
        let time_max_result = jst
            .from_local_datetime(&time_max_naive);
        let time_max = match time_max_result {
            chrono::LocalResult::Single(dt) => dt.to_rfc3339(),
            _ => return ["".to_string(), "".to_string()],
        };

        [time_min, time_max]
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let pane = self.pane.render(frame, area);

        let list = self.schedules.iter().map(|schedule| {
            schedule.value().clone()
        })
        .collect();

        self.list.render(frame, pane, list);
    }

    pub fn open(&self) {
        let schedule: String = self.schedules[self.list.index].link.to_string();

        Command::new(BROWSER_PATH)
            .arg(schedule)
            .output()
            .expect("Failed to execute open_schedule");
    }

    pub fn add(&mut self, token_info: &TokenInfo) {
        let client = Client::new();

        let event = json!({
            "summary": self.form.summary.text.to_string(),
            "description": self.form.description.text.to_string(),
            "start": {
                "dateTime": format!("{}+09:00", self.form.start.text.to_string()), 
                "timeZone": "Asia/Tokyo".to_string(),
            },
            "end": {
                "dateTime": format!("{}+09:00", self.form.end.text.to_string()), 
                "timeZone": "Asia/Tokyo".to_string(),
            },
        });

        let _ = client
            .post(BASE_API)
            .bearer_auth(&token_info.access_token)
            .json(&event)
            .send();

        self.schedules = Self::read(token_info);

        self.form.popup.active = false;
        self.form.all_clear();
    }

    pub fn edit(&mut self, token_info: &TokenInfo) {
        let client = Client::new();

        let schedule = &self.schedules[self.list.index];

        let event = json!({
            "summary": self.form.summary.text.to_string(),
            "description": self.form.description.text.to_string(),
            "start": {
                "dateTime": self.form.start.text.to_string(), 
                "timeZone": "Asia/Tokyo".to_string(),
            },
            "end": {
                "dateTime": self.form.end.text.to_string(), 
                "timeZone": "Asia/Tokyo".to_string(),
            },
        });

        let url = format!("{}/{}", BASE_API, schedule.id);

        let _ = client
            .put(url)
            .bearer_auth(&token_info.access_token)
            .json(&event)
            .send();

        self.schedules = Self::read(token_info);

        self.form.popup.active = false;
        self.form.all_clear();
    }

    pub fn delete(&mut self, token_info: &TokenInfo) {
        let client = Client::new();

        let schedule = &self.schedules[self.list.index];

        let url = format!("{}/{}", BASE_API, schedule.id);

        let _ = client
            .delete(url)
            .bearer_auth(&token_info.access_token)
            .send();

        self.schedules = Self::read(token_info);
    }

    pub fn key_binding(&mut self, key: KeyEvent, token_info: &TokenInfo) {
        match key.code {
            KeyCode::F(1) => {
                if !self.form.popup.active & !self.form.popup.active {
                    self.help.popup.active = true;
                    self.form.popup.active = false;
                }
            },
            KeyCode::F(2) => { 
                if !self.help.popup.active & !self.form.popup.active {
                    self.form.popup.title = "Add Schedule".to_string();
                    self.form.mode = FormMode::New;
                    self.help.popup.active = false;
                    self.form.popup.active = true;
                    self.form.all_clear();
                    self.form.active_summary();
                }
            },
            KeyCode::F(3) => { 
                if !self.help.popup.active & !self.form.popup.active && self.schedules.len() > 0 {
                    self.form.popup.title = "Edit Schedule".to_string();
                    self.form.mode = FormMode::Edit;
                    self.help.popup.active = false;
                    self.form.popup.active = true;
                    self.form.all_clear();
                    self.form.active_summary();

                    let schedule = &self.schedules[self.list.index];
                    self.form.summary.text = schedule.summary.to_string();
                    self.form.start.text = schedule.start.to_string();
                    self.form.end.text = schedule.end.to_string();
                    self.form.description.text = schedule.description.to_string();
                }
            },
            KeyCode::F(12) => {
                if self.form.popup.active {
                    match self.form.mode {
                        FormMode::New => self.add(token_info),
                        FormMode::Edit => self.edit(token_info),
                    }
                }
            }
            KeyCode::Enter => {
                if !self.help.popup.active & !self.form.popup.active {
                    self.open();
                }
            },
            KeyCode::Char('D') => self.delete(token_info),
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
