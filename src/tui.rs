use std::error::Error;
use crossterm::{
    event::{KeyCode},
    terminal::disable_raw_mode,
};
use std::io::Stdout;
use std::sync::mpsc::Receiver;
use crossterm::event::KeyEvent;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use crate::{language::Language, events::{rosary_input_handler, evening_prayer_input_handler}};
use crate::rosary::Rosary;

use crate::language::Language::LATINA;
use crate::render::{redraw, refresh};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MenuItem {
    Rosary,
    EveningPrayer,
    Settings,
    Quit,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Quit => 0,
            MenuItem::Rosary => 1,
            MenuItem::Settings => 2,
            MenuItem::EveningPrayer => 3,
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
    item: MenuItem,
}

impl Window {
    pub fn new() -> Window {
        Window {x: 0, y: 0, lang: LATINA,
                parent_h: 0, parent_w: 0,
                last_error: String::from(""),
                item: MenuItem::Rosary
        }
    }

    pub fn active_menu_item(&self) -> MenuItem {
        return self.item;
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

    pub fn cycle_item(&mut self) {
        self.item = match self.item {
            MenuItem::Rosary => { MenuItem::EveningPrayer }
            MenuItem::EveningPrayer => {MenuItem::Rosary}
            _ => {self.item}
        }
    }
}

pub fn input_handler<'a>(rx: &Receiver<Event<KeyEvent>>,
                         terminal: &mut Terminal<CrosstermBackend<Stdout>>,
                         rosary: &mut Rosary,
                         window: &mut Window) -> Result<MenuItem, Box<dyn Error>> {
    // Input handler
    match rx.recv()? {
        Event::Refresh(_, _) => {
            redraw(terminal, &rosary, window)?;
            Ok(window.active_menu_item())
        },
        Event::Input(event) => {
            match window.active_menu_item() {
                MenuItem::Rosary => rosary_input_handler(&rx, terminal, rosary, window, &event),
                MenuItem::EveningPrayer => evening_prayer_input_handler(&rx, terminal, rosary, window, &event),
                _ => Ok(window.active_menu_item())
            }
        },
        Event::Tick => {Ok(window.active_menu_item())}
    }
}

pub fn key_listen<'a>(rx: &'a Receiver<Event<KeyEvent>>, terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
                      rosary: &'a mut Rosary, window: &'a mut Window)
                      -> MenuItem {
    let new_menu_item = input_handler(&rx, terminal,
                                      rosary, window);
    if new_menu_item.is_err() {
        window.set_error("Unable to read key.".to_owned());
        return window.active_menu_item();
    }
    new_menu_item.unwrap()
}

pub fn center(text: &String, window: &Window) -> String {
    let mut text_width = 0;
    for line in text.lines() {
        if text_width < line.len() {
            text_width = line.len();
        }
    }
    let v_offset = window.get_vert_offset(text_width);
    let offset_string = " ".repeat(v_offset);
    offset_string.clone() + &text.replace("\n", &("\n".to_owned() + &offset_string))
}
