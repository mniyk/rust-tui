use ratatui::Frame;

use crate::app::ui::popup::Popup;

#[derive(Debug)]
pub struct Help {
    pub popup: Popup,
}

impl Help {
    pub fn new(title: &str) -> Self {
        Self {
            popup: Popup::new(title)
        }
    }

    pub fn render(&self, frame: &mut Frame, text: Vec<&str>) {
        if self.popup.active {
            let area = self.popup.render(frame, [80, 80]);

            self.popup.text(frame, area, text, false);
        }
    }
}