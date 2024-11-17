mod bookmark;
mod virtualbox;

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Tabs, Paragraph, List, ListItem},
    DefaultTerminal,
    Frame,
};

use bookmark::Bookmarks;
use virtualbox::VirtualBox;

#[derive(Debug)]
enum WindowMode {
    Bookmark,
    Tab,
}

#[derive(Debug, Clone)]
enum TabMode {
    VirtualBox,
    Tab2,
    Tab3,
}

impl From<TabMode> for Option<usize> {
    fn from(tab: TabMode) -> Self {
        match tab {
            TabMode::VirtualBox => Some(0),
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
    bookmarks: Bookmarks,
    virtualbox: VirtualBox,
}

impl App {
    pub fn new() -> Self {
        Self {
            window_mode: WindowMode::Tab,
            selected_tab: TabMode::VirtualBox,
            tab_labels: vec![
                Line::from("VirtualBox"),
                Line::from("Tab 2"),
                Line::from("Tab 3"),
            ],
            bookmarks: Bookmarks::new(),
            virtualbox: VirtualBox::new(),
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
                                    TabMode::VirtualBox => self.selected_tab = TabMode::Tab2,
                                    TabMode::Tab2 => self.selected_tab = TabMode::Tab3,
                                    TabMode::Tab3 => self.selected_tab = TabMode::VirtualBox,
                                }
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Up => {
                        match self.window_mode {
                            WindowMode::Bookmark => {
                                if self.bookmarks.selected_index > 0 {
                                    self.bookmarks.selected_index -= 1;
                                }
                            }
                            WindowMode::Tab => {
                                match self.selected_tab {
                                    TabMode::VirtualBox => {
                                        if self.virtualbox.selected_index > 0 {
                                            self.virtualbox.selected_index -= 1;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    KeyCode::Down => {
                        match self.window_mode {
                            WindowMode::Bookmark => {
                                if self.bookmarks.selected_index < self.bookmarks.bookmarks.len() - 1 {
                                    self.bookmarks.selected_index += 1;
                                }
                            }
                            WindowMode::Tab => {
                                match self.selected_tab {
                                    TabMode::VirtualBox => {
                                        if self.virtualbox.selected_index < self.virtualbox.machines.len() - 1 {
                                            self.virtualbox.selected_index += 1;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    KeyCode::Enter => {
                        match self.window_mode {
                            WindowMode::Bookmark => {
                                self.bookmarks.open_browser();
                            }
                            WindowMode::Tab => {
                                match self.selected_tab {
                                    TabMode::VirtualBox => self.virtualbox.open_virtualbox(),
                                    _ => {}
                                }
                            }
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
            Constraint::Length(2),
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
        let bookmark_list_item = 
            self.bookmarks.bookmarks.iter().enumerate().map(|(i, bookmark)| {
                if i == self.bookmarks.selected_index {
                    ListItem::new(format!("> {}", bookmark.title.to_string()))
                        .style(Style::default().fg(Color::Green))
                } else {
                    ListItem::new(bookmark.title.clone())
                }
            });

        let bookmarks = match self.window_mode {
            WindowMode::Bookmark => {
                List::new(bookmark_list_item)
                    .block(
                        Block::default()
                            .title("  Bookmark  ")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Green))
                    )
            }
            _ => {
                List::new(bookmark_list_item)
                    .block(
                        Block::default()
                            .title("  Bookmark  ")
                            .borders(Borders::ALL)
                    )
            }
        };
        frame.render_widget(bookmarks, area);
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

        match self.selected_tab {
            // TabMode::VirtualBox => "  VirtualBox  ",
            TabMode::VirtualBox => self.render_virtualbox(frame, app_area),
            TabMode::Tab2 => {},
            TabMode::Tab3 => {},
        }
    }

    fn render_virtualbox(&self, frame: &mut Frame, area: Rect) {
        let machine_list_item = 
            self.virtualbox.machines.iter().enumerate().map(|(i, machine)| {
                if i == self.virtualbox.selected_index {
                    ListItem::new(format!("> {}", machine.to_string()))
                        .style(Style::default().fg(Color::Green))
                } else {
                    ListItem::new(machine.clone())
                }
            });

        let machines = match self.window_mode {
            WindowMode::Tab => {
                List::new(machine_list_item)
                    .block(
                        Block::default()
                            .title("  Virtualbox  ")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Green))
                    )
            }
            _ => {
                List::new(machine_list_item)
                    .block(
                        Block::default()
                            .title("  Virtualbox  ")
                            .borders(Borders::ALL)
                    )
            }
        };
        frame.render_widget(machines, area);
    }

    fn render_footer_area(&self, frame: &mut Frame, area: Rect) {
        let footer_text = Paragraph::new(
            "  Quit: Esc, F1: Bookmark, F2: Tab, Tab: Move Tab\n  Enter: Bookmark -> Open Browser, VirtualBox -> Open Machine",
        );
        frame.render_widget(footer_text, area);
    }
}
