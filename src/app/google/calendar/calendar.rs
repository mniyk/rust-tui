use std::process::Command;

use reqwest::blocking::Client;
use chrono::{Utc, FixedOffset, TimeZone};
use serde::Serialize;

use crate::app::google::authentication::TokenInfo;

const BROWSER_PATH: &str = "/mnt/c/Program Files/BraveSoftware/Brave-Browser/Application/brave.exe";

#[derive(Debug)]
pub enum Mode {
    New,
    List,
}

#[derive(Debug)]
pub enum Form {
    Summary,
    Start,
    End,
    Description,
}

#[derive(Debug)]
pub struct Input {
    pub index: usize,
    pub text: String,
}

#[derive(Serialize)]
pub struct Event {
    summary: String,
    description: String,
    start: DateTime,
    end: DateTime,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct DateTime {
    dateTime: String,
    timeZone: String,
}

#[derive(Debug)]
pub struct Schedule {
    pub id: String,
    pub summary: String,
    pub start: String,
    pub end: String,
    pub link: String,
}

#[derive(Debug)]
pub struct Schedules {
    pub selected_index: usize,
    pub schedules: Vec<Schedule>,
    pub mode: Mode,
    pub selected_form: Form,
    pub summary: Input,
    pub start: Input,
    pub end: Input,
    pub description: Input,
}

impl Schedules {
    pub fn new(token_info: &TokenInfo) -> Self {
        Self {
            selected_index: 0,
            schedules: Self::read_schedules(token_info),
            mode: Mode::List,
            selected_form: Form::Summary,
            summary: Input {
                index: 0,
                text: "".to_string(),
            },
            start: Input {
                index: 0,
                text: "".to_string(),
            },
            end: Input {
                index: 0,
                text: "".to_string(),
            },
            description: Input {
                index: 0,
                text: "".to_string(),
            },
        }
    }
    
    pub fn read_schedules(token_info: &TokenInfo) -> Vec<Schedule> {
        let client = Client::new();
        let url = "https://www.googleapis.com/calendar/v3/calendars/primary/events";

        let [time_min, time_max] = Self::time_min_max();

        let response = client.get(url)
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
                        let start = item["start"]["dateTime"].as_str().or_else(|| item["start"]["date"].as_str()).unwrap_or("No start time");
                        let end = item["end"]["dateTime"].as_str().or_else(|| item["end"]["date"].as_str()).unwrap_or("No end time");
                        let link = item["htmlLink"].to_string().replace("\"", "");

                        let schedule = Schedule {
                            id: id.to_string(),
                            summary: summary.to_string(),
                            start: start.to_string(),
                            end: end.to_string(),
                            link,
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
        
        let time_min_naive = now.date_naive().and_hms_opt(0, 0, 0)
            .expect("Failed to create time_min");
        let time_min_result = jst.from_local_datetime(&time_min_naive);
        let time_min = match time_min_result {
            chrono::LocalResult::Single(dt) => dt.to_rfc3339(),
            _ => return ["".to_string(), "".to_string()],
        };

        let time_max_naive = (now + chrono::Duration::days(1)).date_naive().and_hms_opt(23, 59, 59)
            .expect("Failed to create time_max");
        let time_max_result = jst.from_local_datetime(&time_max_naive);
        let time_max = match time_max_result {
            chrono::LocalResult::Single(dt) => dt.to_rfc3339(),
            _ => return ["".to_string(), "".to_string()],
        };

        [time_min, time_max]
    }

    pub fn open_schedule(&self) {
        let schedule = self.schedules[self.selected_index].link.to_string();

        Command::new(BROWSER_PATH)
            .arg(schedule)
            .output()
            .expect("Failed to execute open_schedule");
    }

    pub fn write_schedule(&self, token_info: &TokenInfo) {
        let url = "https://www.googleapis.com/calendar/v3/calendars/primary/events";

        let client = Client::new();

        let event = Event {
            summary: self.summary.text.to_string(),
            description: self.description.text.to_string(),
            start: DateTime {
                dateTime: self.start.text.to_string(),
                timeZone: "Asia/Tokyo".to_string(),
            },
            end: DateTime {
                dateTime: self.end.text.to_string(),
                timeZone: "Asia/Tokyo".to_string(),
            },
        };

        let _ = client
            .post(url)
            .bearer_auth(&token_info.access_token)
            .json(&event)
            .send();

    }

    pub fn delete_schedule(&self, token_info: &TokenInfo) {
        let schedule = &self.schedules[self.selected_index];

        let url = format!(
            "https://www.googleapis.com/calendar/v3/calendars/primary/events/{}",
            schedule.id
        );

        let client = Client::new();

        let _ = client
            .delete(url)
            .bearer_auth(&token_info.access_token)
            .send();
    }
}