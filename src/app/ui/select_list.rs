use ratatui::{
    crossterm::event::{KeyCode, KeyEvent}, 
    layout::Rect, 
    style::{Color, Style},
    widgets::{List, ListItem, ListState},
    Frame,
};

#[derive(Debug)]
pub struct SelectList<'a> {
    pub index: usize,
    pub list: Vec<ListItem<'a>>,
    pub state: ListState,
    pub items: Vec<String>,
}

impl<'a> SelectList<'a> {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        
        Self {
            index: 0,
            list: Vec::new(),
            state,
            items: Vec::new(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, list: Vec<String>) {
        self.items = list;
        
        // ListItemsを作成
        self.list = self.items.iter().map(|value| {
            ListItem::new(value.clone())
        }).collect();

        // Listウィジェットを作成（ハイライトスタイル付き）
        let list_widget = List::new(self.list.clone())
            .highlight_style(Style::default().fg(Color::Green))
            .highlight_symbol("> ");

        // 現在の選択状態を更新
        self.state.select(Some(self.index));

        // レンダリング
        frame.render_stateful_widget(list_widget, area, &mut self.state);
    }

    pub fn up(&mut self) {
        if self.index > 0 {
            self.index -= 1;
            self.state.select(Some(self.index));
        }
    }

    pub fn right(&mut self) {
        self.down();
    }

    pub fn down(&mut self) {
        if !self.items.is_empty() && self.index < self.items.len() - 1 {
            self.index += 1;
            self.state.select(Some(self.index));
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
