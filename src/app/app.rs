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
use crate::app::task::task::Tasks;
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
    Tasks,
    VirtualBox,
}

impl From<TabMode> for Option<usize> {
    fn from(tab: TabMode) -> Self {
        match tab {
            TabMode::Schedule => Some(0),
            TabMode::Tasks => Some(1),
            TabMode::VirtualBox => Some(2),
        }
    }
}

#[derive(Debug)]
pub struct App<'a> {
    window_mode: WindowMode,
    selected_tab: TabMode,
    tab_labels: Vec<Line<'static>>,
    bookmarks: Bookmarks<'a>,
    schedules: Schedules<'a>,
    tasks: Tasks<'a>,
    virtualbox: VirtualBox<'a>,
    token_info: TokenInfo,
}

impl<'a> App<'a> {
    pub fn new(token_info: &TokenInfo) -> Self {
        Self {
            window_mode: WindowMode::Tab,
            selected_tab: TabMode::Schedule,
            tab_labels: vec![
                Line::from("Schedule"),
                Line::from("Task"),
                Line::from("VirtualBox"),
            ],
            bookmarks: Bookmarks::new(),
            schedules: Schedules::new(token_info), 
            tasks: Tasks::new(token_info),
            virtualbox: VirtualBox::new(),
            token_info: token_info.clone(),
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
                                    self.selected_tab = TabMode::Tasks;
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
                        TabMode::Tasks => match key.code {
                            KeyCode::Tab => {
                                if !self.tasks.help.popup.active & !self.tasks.form.popup.active {
                                    self.selected_tab = TabMode::VirtualBox;
                                }

                                if self.tasks.form.popup.active {
                                    if self.tasks.form.title.active {
                                        self.tasks.form.active_notes();
                                    } else if self.tasks.form.notes.active {
                                        self.tasks.form.active_due();
                                    } else if self.tasks.form.due.active {
                                        self.tasks.form.active_title();
                                    }
                                }
                            },
                            KeyCode::Esc => {
                                if self.tasks.help.popup.active {
                                    self.tasks.help.popup.active = false;
                                } else if self.tasks.form.popup.active {
                                    self.tasks.form.popup.active = false;
                                } else {
                                    break
                                }
                            }
                            _ => self.tasks.key_binding(key, &self.token_info),
                        }
                        TabMode::VirtualBox => match key.code {
                            KeyCode::Tab => {
                                if !self.virtualbox.help.popup.active {
                                    self.selected_tab = TabMode::Schedule;
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
                self.tasks.pane.active = false;
                self.virtualbox.pane.active = false;
            },
            WindowMode::Tab => match self.selected_tab {
                TabMode::Schedule => {
                    self.bookmarks.pane.active = false;
                    self.schedules.pane.active = true;
                    self.tasks.pane.active = false;
                    self.virtualbox.pane.active = false;
                },
                TabMode::Tasks => {
                    self.bookmarks.pane.active = false;
                    self.schedules.pane.active = false;
                    self.tasks.pane.active = true;
                    self.virtualbox.pane.active = false;
                },
                TabMode::VirtualBox => {
                    self.bookmarks.pane.active = false;
                    self.schedules.pane.active = false;
                    self.tasks.pane.active = false;
                    self.virtualbox.pane.active = true;
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
            TabMode::Tasks => self.tasks.render(frame, app_area),
            TabMode::VirtualBox => self.virtualbox.render(frame, app_area),
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
                "Open Schedule           : Enter", 
                "Focus Move Up           : Up, Right", 
                "Focus Move Down         : Down, Left", 
                "Execute Delete Schedule : Shift+D",
                "Open/Close Add Schedule : F2",
                "Open/Close Edit Schedule: F3",
                "",
                "[ Add Schedule ]",
                "Move Input Form          : Tab",
                "Execute Add Schedule     : F12",
                "Start/End Datetime Format: yyyy/mm/ddThh:mm:ss",
                "",
                "[ Edit Schedule ]",
                "Move Input Form          : Tab",
                "Execute Edit Schedule    : F12",
                "Start/End Datetime Format: yyyy/mm/ddThh:mm:ss+09:00",
            ]
        );
        self.schedules.form.render(frame);

        self.tasks.help.render(
            frame,
            vec![
                "[ Select Task ]",
                "Open Task            : Enter", 
                "Focus Move Up        : Up, Right", 
                "Focus Move Down      : Down, Left", 
                "Execute Complete Task: Shift+C",
                "Execute Delete Task  : Shift+D",
                "Open/Close Add Task  : F2",
                "Open/Close Edit Task : F3",
                "",
                "[ Add Task ]",
                "Move Input Form          : Tab",
                "Execute Add Task         : F12",
                "Start/End Datetime Format: yyyy/mm/ddThh:mm:ss",
                "",
                "[ Edit Task ]",
                "Move Input Form          : Tab",
                "Execute Edit Task        : F12",
                "Start/End Datetime Format: yyyy/mm/ddThh:mm:ss.000Z",
            ]
        );
        self.tasks.form.render(frame);

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
