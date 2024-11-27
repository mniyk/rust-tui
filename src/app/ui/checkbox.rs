use ratatui::{
    crossterm::event::{KeyCode, KeyEvent}, 
    layout::{Constraint, Layout, Rect}, 
    widgets::Paragraph, 
    Frame,
};

pub struct CheckBox {
    check: bool,
    label: String,
}

impl CheckBox {
    pub fn new(label: &str) -> Self {
        Self {
            check: false,
            label: label.to_string(),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::horizontal([
            Constraint::Length(3),
            Constraint::Min(0),
        ]);
        let [check_area, label_area] = layout.areas(area);

        let label = Paragraph::new(self.label.to_string());
        frame.render_widget(label, label_area);

        let check = if self.check {
            Paragraph::new("☑")
        } else {
            Paragraph::new("☐")
        };
        frame.render_widget(check, check_area);
    }

    pub fn checked(&mut self) {
        self.check = true;
    }
}

pub struct CheckBoxes {
    pub index: usize,
    pub checkboxes: Vec<CheckBox>,
}

impl CheckBoxes {
    pub fn new() -> Self {
        Self {
            index: 0,
            checkboxes: Vec::new(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, list: Vec<&str>, vertical: bool) {
        let constraints: Vec<Constraint> = list
            .iter().map(|_| Constraint::Length(100 / list.len() as u16)).collect();

        let layout = if vertical {
            Layout::vertical(constraints)
        } else {
            Layout::horizontal(constraints)
        };

        self.checkboxes = list 
            .iter()
            .map(|label| CheckBox::new(label))
            .collect();

        layout.split(area).iter().enumerate().for_each(|(i, checkbox_area)| {
            let checkbox = &mut self.checkboxes[i];

            if self.index == i {
                checkbox.checked();
            }
            checkbox.render(frame, *checkbox_area);
        });
    }

    pub fn up(&mut self) {
        self.index = self.index.saturating_sub(1).clamp(0, self.checkboxes.len() - 1);
    }

    pub fn right(&mut self) {
        self.down();
    }

    pub fn down(&mut self) {
        self.index = self.index.saturating_add(1).clamp(0, self.checkboxes.len() - 1);
    }

    pub fn left(&mut self) {
        self.up();
    }

    pub fn clear(&mut self) {
        self.index = 0;
    }

    pub fn key_binding(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.up(),
            KeyCode::Right => self.right(),
            KeyCode::Down => self.down(),
            KeyCode::Left => self.left(),
            _ => {},
        }
    }
}