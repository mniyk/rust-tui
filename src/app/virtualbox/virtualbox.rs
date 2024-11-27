use core::str;
use std::process::Command;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect, 
    Frame,
};

use crate::app::ui::{
    help::Help,
    pane::Pane,
    select_list::SelectList,
};

const VIRTUALBOX_PATH: &str = "/mnt/c/Program Files/Oracle/VirtualBox/VBoxManage.exe";
const APP_TITLE: &str = "VirtualBox";
const HELP_TITLE: &str = "Help VirtualBox";

#[derive(Debug)]
pub struct VirtualBox<'a> {
    pub pane: Pane,
    pub machines: Vec<String>,
    pub select_list: SelectList<'a>,
    pub help: Help,
}

impl<'a> VirtualBox<'a> {
    pub fn new() -> Self {
        Self {
            pane: Pane::new(APP_TITLE),
            machines: Self::read(),
            select_list: SelectList::new(),
            help: Help::new(HELP_TITLE),
        }
    }

    fn read() -> Vec<String> {
        let output = Command::new(VIRTUALBOX_PATH)
            .arg("list")
            .arg("vms")
            .output()
            .expect("Failed to execute read_machines");

        let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");

        output_str
            .lines()
            .filter_map(|line| {
                if let Some(start) = line.find('"') {
                    if let Some(end) = line[start + 1..].find('"') {
                        Some(line[start + 1..start + 1 + end].to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let pane = self.pane.render(frame, area);

        self.select_list.render(frame, pane, self.machines.clone());
    }

    pub fn open(&self) {
        let machine = self.machines[self.select_list.index].to_string();

        Command::new(VIRTUALBOX_PATH)
            .arg("startvm")
            .arg(machine)
            .arg("--type")
            .arg("gui")
            .output()
            .expect("Failed to execute open_virtualbox");
    }

    pub fn key_binding(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::F(1) => self.help.popup.active = true,
            KeyCode::Enter => self.open(),
            _ => self.select_list.key_binding(key),
        }
    }
}