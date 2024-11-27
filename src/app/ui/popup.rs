use ratatui::{
    layout::{Constraint, Flex, Layout, Margin, Rect}, 
    style::{Color, Style}, 
    text::{Line, Span}, 
    widgets::{Block, Clear, Paragraph}, 
    Frame,
};

#[derive(Debug)]
pub struct Popup {
    title: String,
    pub active: bool,
}

impl Popup {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            active: false,
        }
    }

    pub fn render(&self, frame: &mut Frame, percent: [u16; 2]) -> Rect {
        let horizontal = Layout::horizontal([
            Constraint::Percentage(percent[0])
        ])
        .flex(Flex::Center);
        let [area] = horizontal.areas(frame.area());

        let vertical = Layout::vertical([
            Constraint::Percentage(percent[1])
        ])
        .flex(Flex::Center);
        let [area] = vertical.areas(area);

        frame.render_widget(Clear, area);

        let block = if self.active {
            Block::bordered()
                .border_style(Style::new().fg(Color::Green)).title(format!("  {}  ", self.title))
        } else {
            Block::bordered().title(format!("  {}  ", self.title))
        };
        frame.render_widget(block, area);

        area
    }

    pub fn text(&self, frame: &mut Frame, area: Rect, text: Vec<&str>, center: bool) {
        let text_box: Vec<Line> = text.iter().map(|&line| {
            Line::from(Span::raw(line))
        })
        .collect();
        let text_box = if center {
            Paragraph::new(text_box).centered()
        } else {
            Paragraph::new(text_box)
        };

        let text_area = area.inner(Margin::new(2, 1));
        frame.render_widget(text_box, text_area);
    }
}
