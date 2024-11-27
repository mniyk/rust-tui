use ratatui::{
    crossterm::event::{KeyCode, KeyEvent}, 
    layout::Rect, 
    style::{Color, Style},
    widgets::{List, ListItem},
    Frame,
};

#[derive(Debug)]
pub struct SelectList<'a> {
    pub index: usize,
    pub list: Vec<ListItem<'a>>,
}

impl<'a> SelectList<'a> {
    pub fn new() -> Self {
        Self {
            index: 0,
            list: Vec::new(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, list: Vec<String>) {
        self.list = list.iter().enumerate().map(|(i, value)| {
            if i == self.index {
                ListItem::new(format!("> {}", value.clone()))
                    .style(Style::new().fg(Color::Green))
            } else {
                ListItem::new(value.clone())
            }
        })
        .collect();
        let list = List::new(self.list.clone());

        frame.render_widget(list, area);
    }

    pub fn up(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn right(&mut self) {
        self.down();
    }

    pub fn down(&mut self) {
        if self.list.len() > 0 {
            if self.index < self.list.len() - 1 {
                self.index += 1;
            }
        }
    }

    pub fn left(&mut self) {
        self.up();
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