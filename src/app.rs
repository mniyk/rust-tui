use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Tabs, Paragraph},
    DefaultTerminal,
    Frame,
};

#[derive(Debug)]
enum WindowMode {
    Bookmark,
    Tab,
}

#[derive(Debug, Clone)]
enum TabMode {
    Tab1,
    Tab2,
    Tab3,
}

impl From<TabMode> for Option<usize> {
    fn from(tab: TabMode) -> Self {
        match tab {
            TabMode::Tab1 => Some(0),
            TabMode::Tab2 => Some(1),
            TabMode::Tab3 => Some(2),
        }
    }
}

#[derive(Debug)]
pub struct App {
    window_mode: WindowMode,
    selected_tab: TabMode,
    tab_labels: Vec<Line<'static>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window_mode: WindowMode::Tab,
            selected_tab: TabMode::Tab1,
            tab_labels: vec![
                Line::from("Tab 1"),
                Line::from("Tab 2"),
                Line::from("Tab 3"),
            ],
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::F(1) => self.window_mode = WindowMode::Bookmark,
                    KeyCode::F(2) => self.window_mode = WindowMode::Tab,
                    KeyCode::Tab => {
                        match self.window_mode {
                            WindowMode::Tab => {
                                match self.selected_tab {
                                    TabMode::Tab1 => self.selected_tab = TabMode::Tab2,
                                    TabMode::Tab2 => self.selected_tab = TabMode::Tab3,
                                    TabMode::Tab3 => self.selected_tab = TabMode::Tab1,
                                }
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let main_layout = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [app_area, footer_area] = main_layout.areas(frame.area());

        let app_layout = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ]);
        let [bookamrk_area, tab_area] = app_layout.areas(app_area);

        self.render_bookmark_area(frame, bookamrk_area);

        self.render_tab_area(frame, tab_area);

        self.render_footer_area(frame, footer_area);
    }

    fn render_bookmark_area(&self, frame: &mut Frame, area: Rect) {
        let bookmark = match self.window_mode {
            WindowMode::Bookmark => {
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green))
                    .title(" Bookmark ")
            },
            _ => {
                Block::default().borders(Borders::ALL).title(" Bookmark ")
            }
        };
        frame.render_widget(bookmark, area);
    }

    fn render_tab_area(&self, frame: &mut Frame, area: Rect) {
        let tab_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(0),
        ]);
        let [header_area, app_area] = tab_layout.areas(area);

        let tabs = Tabs::new(self.tab_labels.clone())
            .select(self.selected_tab.clone())
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Green))
            .block(Block::default().borders(Borders::ALL).title("  Tabs  "));
        frame.render_widget(tabs, header_area);

        let content = match self.selected_tab {
            TabMode::Tab1 => "  Content of Tab 1  ",
            TabMode::Tab2 => "  Content of Tab 2  ",
            TabMode::Tab3 => "  Content of Tab 3  ",
        };

        let content_block = match self.window_mode {
            WindowMode::Tab => {
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green))
                    .title(content)
            }
            _ => {
                Block::default()
                    .borders(Borders::ALL)
                    .title(content)
            }
        };
        frame.render_widget(content_block, app_area);
    }

    fn render_footer_area(&self, frame: &mut Frame, area: Rect) {
        let footer_text = Paragraph::new(
            "  Quit: Esc, F1: Bookmark, F2: Tab, Tab: Move Tab"
        );
        frame.render_widget(footer_text, area);
    }
}
