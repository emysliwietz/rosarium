use std::error::Error;
use chrono::prelude::*;
use crossterm::{
    event::{self as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::Stdout;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
use crossterm::event::KeyEvent;
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};
use tui::layout::Constraint::Percentage;
use tui::symbols::line::{CROSS, THICK_CROSS};
use tui::text::Text;
use tui::widgets::Wrap;
use crate::language::{get_title_translation, Language};
use crate::rosary::{get_daily_mystery, Rosary};
use crate::rosary::Prayer::{FirstMystery, FourthMystery, SecondMystery, ThirdMystery};

use crate::language::Language::LATINA;
use crate::render::{redraw, refresh, render_prayer, render_progress};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MenuItem {
    Rosary,
    Settings,
    Quit,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Quit => 0,
            MenuItem::Rosary => 1,
            MenuItem::Settings => 2
        }
    }
}

pub enum Event<I> {
    Input(I),
    Refresh(u16, u16),
    Tick,
}

#[derive(Debug)]
pub struct Window {
    x: u16,
    y: u16,
    lang: Language,
    parent_h: u16,
    parent_w: u16,
    last_error: String,
}

impl Window {
    pub fn new() -> Window {
        Window {x: 0, y: 0, lang: LATINA, parent_h: 0, parent_w: 0, last_error: String::from("")}
    }

    pub fn get_offset(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    /// Return offset at the top of window for the content to be centered vertically
    pub fn get_top_offset(&self, content_height: usize) -> usize {
        if content_height >= self.parent_h as usize {
            0
        } else {
            ((self.parent_h as usize - content_height) / 2) as usize
        }
    }

    /// Return offset at the top of window for the content to be centered vertically
    pub fn get_vert_offset(&self, content_width: usize) -> usize {
        if content_width >= self.parent_w as usize {
            0
        } else {
            ((self.parent_w as usize - content_width) / 2) as usize
        }
    }

    pub fn down(&mut self) {
        if self.x != 0 {
            self.x -= 1;
        }
    }

    pub fn up(&mut self) {
        if self.x != u16::MAX {
            self.x += 1;
        }
    }

    pub fn left(&mut self) {
        if self.y != 0 {
            self.y -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.y != u16::MAX {
            self.y += 1;
        }
    }

    pub fn set_parent_dims(&mut self, w: u16, h: u16) {
        self.parent_w = w;
        self.parent_h = h;
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn language(&self) -> String {
        self.lang.to_string()
    }

    pub fn has_error(&self) -> bool {
        self.last_error != String::from("")
    }

    pub fn error(&self) -> String {
        self.last_error.clone()
    }

    pub fn clear_error(&mut self) {
        self.last_error = String::from("");
    }

    pub fn set_error(&mut self, error: String) {
        self.last_error = error;
    }

    pub fn cycle_language(&mut self) {
        match self.lang {
            Language::ANGLIA => { self.lang = Language::GERMANA; }
            Language::GERMANA => { self.lang = Language::LATINA; }
            LATINA => {self.lang = Language::ANGLIA; }
        }
    }
}

pub fn input_handler<'a>(rx: &Receiver<Event<KeyEvent>>, terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
                         rosary: &'a mut Rosary,
                         current_menu_item: &'a MenuItem,
                         window: &'a mut Window) -> Result<&'a MenuItem, Box<dyn Error>> {
    // Input handler
    match rx.recv()? {
        Event::Refresh(_, _) => {
            redraw(terminal, &rosary, window)?
        },
        Event::Input(event) => {
            match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    return Ok(&MenuItem::Quit);
                }
                KeyCode::Char('r') => refresh(terminal, &rosary, window),
                KeyCode::Char('s') => return Ok(&MenuItem::Settings),
                KeyCode::Char(' ') => advance(rosary, window),
                KeyCode::Char('l') => advance(rosary, window),
                KeyCode::Char('h') => recede(rosary, window),
                KeyCode::Char('k') => scroll_down(window, &rosary),
                KeyCode::Char('j') => scroll_up(window, &rosary),
                KeyCode::Char('x') => window.cycle_language(),
                KeyCode::Char('H') => scroll_left(window, &rosary),
                KeyCode::Char('L') => scroll_right(window, &rosary),
                KeyCode::Right => advance(rosary, window, ),
                KeyCode::Backspace => recede(rosary, window),
                KeyCode::Left => recede(rosary, window),
                KeyCode::Up => {}
                _ => {}
            }
            redraw(terminal, &rosary, window)?;
        },
        Event::Tick => {}
    };
    return Ok(current_menu_item);
}

pub fn key_listen<'a>(rx: &'a Receiver<Event<KeyEvent>>, terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
                      rosary: &'a mut Rosary, window: &'a mut Window, active_menu_item: &'a mut MenuItem)
                      -> MenuItem {
    let new_menu_item = input_handler(&rx, terminal,
                                      rosary, &active_menu_item, window);
    if new_menu_item.is_err() {
        window.set_error("Unable to read key.".to_owned());
        return active_menu_item.clone();
    }
    let mut new_menu_item = new_menu_item.unwrap();
    return *new_menu_item;
}

fn advance(rosary: &mut Rosary, window: &mut Window) {
    rosary.advance();
    render_progress(&rosary, window);
}

fn recede(rosary: &mut Rosary, window: &mut Window) {
    rosary.recede();
    render_progress(&rosary, window);
}

fn scroll_down(window: &mut Window, rosary: &Rosary) {
    window.down();
    render_prayer(rosary, window);
}

fn scroll_up(window: &mut Window, rosary: &Rosary) {
    window.up();
    render_prayer(rosary, window);
}

fn scroll_left(window: &mut Window, rosary: &Rosary) {
    window.left();
    render_prayer(rosary, window);
}

fn scroll_right(window: &mut Window, rosary: &Rosary) {
    window.right();
    render_prayer(rosary, window);
}

pub fn center(text: &String, window: &Window) -> String {
    let mut text_width = 0;
    for line in text.lines() {
        if text_width < line.len() {
            text_width = line.len();
        }
    }
    let v_offset = window.get_vert_offset(text_width);
    let offset_string = (" ".repeat(v_offset));
    offset_string.clone() + &text.replace("\n", &("\n".to_owned() + &offset_string))
}