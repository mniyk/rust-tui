mod bookmark;
mod virtualbox;
pub mod google;

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    DefaultTerminal,
    Frame,
};

use bookmark::Bookmarks;
use virtualbox::VirtualBox;
use google::authentication::TokenInfo;
use google::calendar::{
    calendar::{
        Schedules,
        Mode as ScheduleMode,
    },
    ui::UI as CalendarUI,
    keybind::KeyBind as CalendarKeyBind,
};

#[derive(Debug)]
pub enum WindowMode {
    Bookmark,
    Tab,
}

#[derive(Debug, Clone)]
pub enum TabMode {
    Schedule,
    VirtualBox,
    Tab3,
}

impl From<TabMode> for Option<usize> {
    fn from(tab: TabMode) -> Self {
        match tab {
            TabMode::Schedule => Some(0),
            TabMode::VirtualBox => Some(1),
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
    token_info: TokenInfo,
    schedules: Schedules,
}

impl App {
    pub fn new(token_info: &TokenInfo) -> Self {
        Self {
            window_mode: WindowMode::Tab,
            selected_tab: TabMode::Schedule,
            tab_labels: vec![
                Line::from("Schedule"),
                Line::from("VirtualBox"),
                Line::from("Tab 3"),
            ],
            bookmarks: Bookmarks::new(),
            virtualbox: VirtualBox::new(),
            token_info: token_info.clone(),
            schedules: Schedules::new(token_info), 
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::F(1) => self.window_mode = WindowMode::Bookmark,
                    KeyCode::F(2) => self.window_mode = WindowMode::Tab,
                    _ => {},
                }
                
                match self.window_mode {
                    WindowMode::Bookmark => match key.code {
                        KeyCode::Up => {
                            if self.bookmarks.selected_index > 0 {
                                self.bookmarks.selected_index -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.bookmarks.selected_index < self.bookmarks.bookmarks.len() - 1 {
                                self.bookmarks.selected_index += 1;
                            }
                        }
                        KeyCode::Enter => self.bookmarks.open_browser(),
                        KeyCode::Esc => break,
                        _ => {}
                    },
                    WindowMode::Tab => match self.selected_tab {
                        TabMode::Schedule => match key.code {
                            KeyCode::Esc => match self.schedules.mode {
                                ScheduleMode::List => break,
                                _ => CalendarKeyBind::execute(&self.token_info, key.code, &mut self.selected_tab, &mut self.schedules),
                            },
                            _ => CalendarKeyBind::execute(&self.token_info, key.code, &mut self.selected_tab, &mut self.schedules),
                        }
                        TabMode::VirtualBox => match key.code {
                            KeyCode::Up => {
                                if self.virtualbox.selected_index > 0 {
                                    self.virtualbox.selected_index -= 1;
                                }
                            }
                            KeyCode::Down => {
                                if self.virtualbox.selected_index < self.virtualbox.machines.len() - 1 {
                                    self.virtualbox.selected_index += 1;
                                }
                            }
                            KeyCode::Enter => self.virtualbox.open_virtualbox(),
                            KeyCode::Tab => self.selected_tab = TabMode::Tab3,
                            KeyCode::Esc => break,
                            _ => {}
                        }
                        TabMode::Tab3 => match key.code {
                            KeyCode::Tab => self.selected_tab = TabMode::Schedule,
                            KeyCode::Esc => break,
                            _ => {}
                        }
                    },
                }
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let main_layout = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(4),
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
            TabMode::Schedule => CalendarUI::schedule_area(frame, app_area, &self.window_mode, &self.schedules),
            TabMode::VirtualBox => self.render_virtualbox_area(frame, app_area),
            TabMode::Tab3 => {},
        }
    }


    fn render_virtualbox_area(&self, frame: &mut Frame, area: Rect) {
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
        let line1 = Line::from(vec![
            Span::raw("  Esc: Quit, F1: Bookmark, F2: Tab, Tab: Move Tab"),
        ]);
        let line2 = Line::from(vec![
            Span::raw("  Bookmark   -> Enter: Open Browser"),
        ]);
        let line3 = Line::from(vec![
            Span::raw("  VirtualBox -> Enter: Open Machine"),
        ]);
        let line4 = Line::from(vec![
            Span::raw("  Schedule   -> Enter: Open Google Calendar, Shift+N: New Schedule, Shift+C: Delete Schedule"),
        ]);
        let footer_text = Paragraph::new(
            vec![line1, line2, line3, line4]
        );
        frame.render_widget(footer_text, area);
    }
}
