use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    DefaultTerminal,
    Frame,
};

use crate::app::bookmark::bookmark::Bookmarks;
use crate::app::schedule::schedule::Schedules;
use crate::app::virtualbox::virtualbox::VirtualBox;
use crate::app::google::authentication::TokenInfo;

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
pub struct App<'a> {
    window_mode: WindowMode,
    selected_tab: TabMode,
    tab_labels: Vec<Line<'static>>,
    bookmarks: Bookmarks<'a>,
    virtualbox: VirtualBox<'a>,
    token_info: TokenInfo,
    schedules: Schedules<'a>,
}

impl<'a> App<'a> {
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
        self.schedules.pane.active = true;
        
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                self.change_mode(key);
                
                match self.window_mode {
                    WindowMode::Bookmark => match key.code {
                        KeyCode::Tab => {
                            if self.bookmarks.form.popup.active {
                                if self.bookmarks.form.title.active {
                                    self.bookmarks.form.active_url()
                                } else if self.bookmarks.form.url.active {
                                    self.bookmarks.form.active_title()
                                }
                            }
                        }
                        KeyCode::Esc => {
                            if self.bookmarks.help.popup.active {
                                self.bookmarks.help.popup.active = false;
                            } else if self.bookmarks.form.popup.active {
                                self.bookmarks.form.popup.active = false;
                            } else {
                                break
                            }
                        },
                        _ => self.bookmarks.key_binding(key),
                    }
                    WindowMode::Tab => match self.selected_tab {
                        TabMode::Schedule => match key.code {
                            KeyCode::Tab => {
                                if !self.schedules.help.popup.active & !self.schedules.form.popup.active {
                                    self.selected_tab = TabMode::VirtualBox;
                                }

                                if self.schedules.form.popup.active {
                                    if self.schedules.form.summary.active {
                                        self.schedules.form.active_start();
                                    } else if self.schedules.form.start.active {
                                        self.schedules.form.active_end();
                                    } else if self.schedules.form.end.active {
                                        self.schedules.form.active_description();
                                    } else if self.schedules.form.description.active {
                                        self.schedules.form.active_summary();
                                    }
                                }
                            },
                            KeyCode::Esc => {
                                if self.schedules.help.popup.active {
                                    self.schedules.help.popup.active = false;
                                } else if self.schedules.form.popup.active {
                                    self.schedules.form.popup.active = false;
                                } else {
                                    break
                                }
                            }
                            _ => self.schedules.key_binding(key, &self.token_info),
                        }
                        TabMode::VirtualBox => match key.code {
                            KeyCode::Tab => {
                                if !self.virtualbox.help.popup.active {
                                    self.selected_tab = TabMode::Tab3;
                                }
                            },
                            KeyCode::Esc => {
                                if self.virtualbox.help.popup.active {
                                    self.virtualbox.help.popup.active = false;
                                } else {
                                    break
                                }
                            },
                            _ => self.virtualbox.key_binding(key),
                        }
                        TabMode::Tab3 => match key.code {
                            KeyCode::Tab => self.selected_tab = TabMode::Schedule,
                            KeyCode::Esc => break,
                            _ => {}
                        }
                    },
                }
            
                self.active_pane();
            }
        }

        Ok(())
    }

    fn change_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::F(5) => {
                self.window_mode = WindowMode::Bookmark;
            },
            KeyCode::F(6) => {
                self.window_mode = WindowMode::Tab;
            },
            _ => {},
        }
    }

    fn active_pane(&mut self) {
        match self.window_mode {
            WindowMode::Bookmark => {
                self.bookmarks.pane.active = true;
                self.schedules.pane.active = false;
                self.virtualbox.pane.active = false;
            },
            WindowMode::Tab => match self.selected_tab {
                TabMode::Schedule => {
                    self.bookmarks.pane.active = false;
                    self.schedules.pane.active = true;
                    self.virtualbox.pane.active = false;
                },
                TabMode::VirtualBox => {
                    self.bookmarks.pane.active = false;
                    self.schedules.pane.active = false;
                    self.virtualbox.pane.active = true;
                },
                _ => {
                    self.bookmarks.pane.active = false;
                    self.schedules.pane.active = false;
                    self.virtualbox.pane.active = false;
                },
            },
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [footer_area, bookmark_area, tab_area] = Self::layout(frame);

        self.bookmarks.render(frame, bookmark_area);
        self.tab(frame, tab_area);
        self.footer(frame, footer_area);

        self.popup(frame);
    }

    fn layout(frame: &mut Frame) -> [Rect; 3] {
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

        [footer_area, bookamrk_area, tab_area]
    }

    fn tab(&mut self, frame: &mut Frame, area: Rect) {
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
            TabMode::Schedule => self.schedules.render(frame, app_area),
            TabMode::VirtualBox => self.virtualbox.render(frame, app_area),
            TabMode::Tab3 => {},
        }
    }

    fn footer(&self, frame: &mut Frame, area: Rect) {
        let footer_text = Paragraph::new(
            Line::from(vec![
                Span::raw("  Quit: Esc, Open/Close Help APP: F1, Bookmark: F5, Tab APP: F6, Move Tab APP: Tab"),
            ]),
        );
        frame.render_widget(footer_text, area);
    }

    fn popup(&mut self, frame: &mut Frame) {
        self.bookmarks.help.render(
            frame, 
            vec![
                "[ Select Bookmark ]",
                "Open Bookmark           : Enter", 
                "Focus Move Up           : Up, Right", 
                "Focus Move Down         : Down, Left", 
                "Execute Delete Schedule : Shift+D",
                "Open/Close Add Bookmark : F2",
                "Open/Close Edit Bookmark: F3",
                "",
                "[ Add Bookmark ]",
                "Move Input Form     : Tab", 
                "Execute Add Bookmark: F12", 
                "",
                "[ Edit Bookmark ]",
                "Move Input Form      : Tab", 
                "Execute Edit Bookmark: F12", 
            ]
        );
        self.bookmarks.form.render(frame);

        self.schedules.help.render(
            frame,
            vec![
                "[ Select Schedule ]",
                "Open Schedule  : Enter", 
                "Focus Move Up  : Up, Right", 
                "Focus Move Down: Down, Left", 
                "F2             : Open/Close Add Schedule",
                "",
                "[ Add Schedule ]",
                "Move Input Form          : F2",
                "Execute Add Schedule     : F12",
                "Execute Delete Schedule  : Shift+D",
                "Start/End Datetime Format: yyyy/mm/ddThh/mm/ss",
            ]
        );
        self.schedules.form.render(frame);

        self.virtualbox.help.render(
            frame,
            vec![
                "[ Select VirtualBox ]",
                "Open Machine   : Enter", 
                "Focus Move Up  : Up, Right", 
                "Focus Move Down: Down, Left", 
            ]
        );
    }
}
