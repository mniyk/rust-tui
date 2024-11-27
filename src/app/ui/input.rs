use ratatui::{
    crossterm::event::{KeyCode, KeyEvent}, 
    layout::{Position, Rect}, 
    widgets::Paragraph,
    Frame,
};

#[derive(Debug)]
pub struct Input {
    pub index: usize,
    pub text: String,
    pub active: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            index: 0,
            text: String::new(),
            active: false,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let input = Paragraph::new(self.text.as_str());

        if self.active {
            frame.set_cursor_position(Position::new(area.x + self.index as u16, area.y));
        }

        frame.render_widget(input, area);
    }

    pub fn input(&mut self, char: char) {
        let index = self.text
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.index)
            .unwrap_or(self.text.len());

        self.text.insert(index, char);

        self.right();
    }

    pub fn right(&mut self) {
        self.index = self.index
            .saturating_add(1)
            .clamp(0, self.text.chars().count());
    }

    pub fn left(&mut self) {
        self.index = self.index
            .saturating_sub(1)
            .clamp(0, self.text.chars().count());
    }

    pub fn clear(&mut self) {
        self.index = 0;
        self.text.clear();
    }

    pub fn delete(&mut self) {
        if self.index != 0 {
            let before_text = self.text.chars().take(self.index - 1);
            let after_text = self.text.chars().skip(self.index);

            self.text = before_text.chain(after_text).collect();

            self.left();
        }
    }

    pub fn key_binding(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Backspace => self.delete(),
            KeyCode::Left => self.left(),
            KeyCode::Right => self.right(),
            KeyCode::Char(char) => self.input(char),
            _ => {},
        }
    }
}