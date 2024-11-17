use core::str;
use std::process::Command;

const VIRTUALBOX_PATH: &str = "/mnt/c/Program Files/Oracle/VirtualBox/VBoxManage.exe";

#[derive(Debug)]
pub struct VirtualBox {
    pub selected_index: usize,
    pub machines: Vec<String>,
}

impl VirtualBox {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            machines: Self::read_machines(),
        }
    }

    fn read_machines() -> Vec<String> {
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

    pub fn open_virtualbox(&self) {
        let machine = self.machines[self.selected_index].to_string();

        Command::new(VIRTUALBOX_PATH)
            .arg("startvm")
            .arg(machine)
            .arg("--type")
            .arg("gui")
            .output()
            .expect("Failed to execute open_virtualbox");
    }
}