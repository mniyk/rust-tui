use std::process::Command;

use reqwest::blocking::Client;
use chrono::{Utc, FixedOffset, TimeZone};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent}, layout::Rect, widgets::block::Title, Frame
};
use serde::Serialize;
use serde_json::json;

use crate::app::task::form::{Form, Mode as FormMode};
use crate::app::ui::{
    help::Help,
    select_list::SelectList,
    pane::Pane,
};
use crate::app::google::authentication::TokenInfo;

const BASE_API: &str = "https://tasks.googleapis.com/tasks/v1/lists/@default/tasks";
const BROWSER_PATH: &str = "/mnt/c/Program Files/BraveSoftware/Brave-Browser/Application/brave.exe";
const APP_TITLE: &str = "Task";
const HELP_TITLE: &str = "Help Schedule";

#[derive(Debug, Serialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub notes: String,
    pub due: String,
    pub status: String,
}

impl Task {
    fn value(&self) -> String {
        format!("{}\n  {}\n  {}", self.title, self.notes, self.due)
    }
}

#[derive(Debug)]
pub struct Tasks<'a> {
    pub pane: Pane,
    pub tasks: Vec<Task>,
    pub list: SelectList<'a>,
    pub form: Form,
    pub help: Help,
}

impl<'a> Tasks<'a> {
    pub fn new(token_info: &TokenInfo) -> Self {
        Self {
            pane: Pane::new(APP_TITLE),
            tasks: Self::read(token_info),
            list: SelectList::new(),
            form: Form::new() ,
            help: Help::new(HELP_TITLE),
        }
    }

    pub fn read(token_info: &TokenInfo) -> Vec<Task> {
        let client = Client::new();

        let response = client
            .get(BASE_API)
            .bearer_auth(&token_info.access_token)
            .query(&[("showCompleted", "false")])
            .send()
            .unwrap();

        let mut tasks = Vec::<Task>::new();
        if response.status().is_success() {
            let events: serde_json::Value = response.json().unwrap();
            if let Some(items) = events["items"].as_array() {
                if !items.is_empty() {
                    for item in items {
                        let id = item["id"].to_string().replace("\"", "");
                        let title = item["title"].to_string().replace("\"", "");
                        let notes = item["notes"].to_string().replace("\"", "");
                        let due = item["due"].to_string().replace("\"", "");
                        let status = item["status"].to_string().replace("\"", "");

                        let task: Task = Task { id, title, notes, due, status };

                        tasks.push(task);
                    }
                }
            }
        };

        tasks
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let pane = self.pane.render(frame, area);

        let list = self.tasks.iter().map(|tasks| {
            tasks.value().clone()
        })
        .collect();

        self.list.render(frame, pane, list);
    }

    pub fn add(&mut self, token_info: &TokenInfo) {
        let client = Client::new();

        let event = json!({
            "title": self.form.title.text.to_string(),
            "notes": self.form.notes.text.to_string(),
            "due": format!("{}.000Z", self.form.due.text.to_string()),
        });

        let _ = client
            .post(BASE_API)
            .bearer_auth(&token_info.access_token)
            .json(&event)
            .send();

        self.tasks = Self::read(token_info);

        self.form.popup.active = false;
        self.form.all_clear();
    }

    pub fn edit(&mut self, token_info: &TokenInfo) {
        let client = Client::new();

        let task = &mut self.tasks[self.list.index];

        task.title = self.form.title.text.to_string();
        task.notes = self.form.notes.text.to_string();
        task.due = self.form.due.text.to_string();

        let url = format!("{}/{}", BASE_API, task.id);

        let _ = client
            .put(url)
            .bearer_auth(&token_info.access_token)
            .json(&task)
            .send();


        self.tasks = Self::read(token_info);

        self.form.popup.active = false;
        self.form.all_clear();
    }

    pub fn open(&self) {
        Command::new(BROWSER_PATH)
            .arg("https://calendar.google.com/calendar/u/0/r/tasks")
            .output()
            .expect("Failed to execute open_schedule");
    }

    pub fn complete(&mut self, token_info: &TokenInfo) {
        let client = Client::new();

        let task = &mut self.tasks[self.list.index];
        task.status = "completed".to_string();

        let url = format!("{}/{}", BASE_API, task.id);

        let _ = client
            .put(url)
            .bearer_auth(&token_info.access_token)
            .json(&task)
            .send();
        
        self.tasks = Self::read(token_info);

        self.form.popup.active = false;
        self.form.all_clear();
    }

    pub fn delete(&mut self, token_info: &TokenInfo) {
        let client = Client::new();

        let task = &self.tasks[self.list.index];

        let url = format!("{}/{}", BASE_API, task.id);

        let _ = client
            .delete(url)
            .bearer_auth(&token_info.access_token)
            .send();

        self.tasks = Self::read(token_info);
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
                    self.form.popup.title = "Add Task".to_string();
                    self.form.mode = FormMode::New;
                    self.help.popup.active = false;
                    self.form.popup.active = true;
                    self.form.all_clear();
                    self.form.active_title();
                }
            },
            KeyCode::F(3) => { 
                if !self.help.popup.active & !self.form.popup.active && self.tasks.len() > 0 {
                    self.form.popup.title = "Edit Task".to_string();
                    self.form.mode = FormMode::Edit;
                    self.help.popup.active = false;
                    self.form.popup.active = true;
                    self.form.all_clear();
                    self.form.active_title();

                    let schedule = &self.tasks[self.list.index];
                    self.form.title.text = schedule.title.to_string();
                    self.form.notes.text = schedule.notes.to_string();
                    self.form.due.text = schedule.due.to_string();
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
            KeyCode::Char('C') => self.complete(token_info),
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


use std::fs::OpenOptions;
use std::io::Write;

fn send_terminal(message: String) {
    let terminal_path = "/dev/pts/3";

    match OpenOptions::new().write(true).open(terminal_path) {
        Ok(mut terminal) => {
            if let Err(e) =  writeln!(terminal, "{:?}", message) {
                eprintln!("Failed to write to {}: {}", terminal_path, e);
            }
        }
        Err(e) => {
            eprintln!("Failed to open {}: {}", terminal_path, e);
        }
    }
}