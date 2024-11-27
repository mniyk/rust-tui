use ratatui::{
    layout::Rect, 
    style::{Color, Style}, 
    widgets::Block, 
    Frame,
};

#[derive(Debug)]
pub struct Pane {
    title: String,
    pub active: bool,
}

impl Pane {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            active: false,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) -> Rect {
        let outer = if self.active {
            Block::bordered()
                .border_style(Style::new().fg(Color::Green))
                .title(format!("  {}  ", self.title))
        } else {
            Block::bordered().title(format!("  {}  ", self.title))
        };
        let inner = outer.inner(area);
        frame.render_widget(outer, area);

        inner
    }
}