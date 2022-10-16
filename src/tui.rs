use crate::prayer::EveningPrayer;
use crate::rosary::Rosary;
use crate::{
    events::{evening_prayer_input_handler, rosary_input_handler},
    language::Language,
};
use crossterm::event::KeyEvent;
use crossterm::{event::KeyCode, terminal::disable_raw_mode};
use std::error::Error;
use std::io::Stdout;
use std::sync::mpsc::Receiver;
use tui::{backend::CrosstermBackend, Terminal};

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

#[derive(Debug, Eq, PartialEq)]
pub enum WindowStack {
    HSplit(Box<WindowStack>, Box<WindowStack>),
    VSplit(Box<WindowStack>, Box<WindowStack>),
    Node(Window),
}

pub struct Frame {
    pub ws: WindowStack,
}

fn new_ws_box() -> Box<WindowStack> {
    Box::from(WindowStack::Node(Window::new()))
}

pub fn _get_active_window(s: &mut WindowStack) -> Option<&mut Window> {
    match s {
        WindowStack::Node(w) => {
            if w.is_active {
                return Some(w);
            }
        }
        WindowStack::HSplit(v, w) => {
            let v = _get_active_window(v);
            let w = _get_active_window(w);
            if v.is_some() {
                return v;
            } else if w.is_some() {
                return w;
            }
        }
        WindowStack::VSplit(v, w) => {
            let v = _get_active_window(v);
            let w = _get_active_window(w);
            if v.is_some() {
                return v;
            } else if w.is_some() {
                return w;
            }
        }
    };
    return None;
}

impl Frame {
    pub fn new() -> Frame {
        let mut w = Window::new();
        w.is_active = true;
        let w = WindowStack::Node(w);
        Frame { ws: w }
    }

    pub fn get_active_window(&mut self) -> &mut Window {
        return _get_active_window(&mut self.ws).unwrap();
    }

    pub fn vsplit(mut self) {
        self.ws = WindowStack::VSplit(Box::from(self.ws), new_ws_box())
    }

    pub fn hsplit(mut self) {
        self.ws = WindowStack::HSplit(Box::from(self.ws), new_ws_box())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Window {
    x: u16,
    y: u16,
    lang: Language,
    parent_h: u16,
    parent_w: u16,
    last_error: String,
    item: MenuItem,
    pub is_active: bool,
    pub rosary: Rosary,
    pub evening_prayer: EveningPrayer,
}

impl Window {
    pub fn new() -> Window {
        Window {
            x: 0,
            y: 0,
            lang: LATINA,
            parent_h: 0,
            parent_w: 0,
            last_error: String::from(""),
            item: MenuItem::Rosary,
            is_active: false,
            rosary: Rosary::new(),
            evening_prayer: EveningPrayer::new(),
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
            Language::ANGLIA => {
                self.lang = Language::GERMANA;
            }
            Language::GERMANA => {
                self.lang = Language::LATINA;
            }
            LATINA => {
                self.lang = Language::ANGLIA;
            }
            _ => {
                self.lang = Language::GERMANA;
            }
        }
    }

    pub fn set_language(&mut self, l: &Language) {
        self.lang = (*l).clone()
    }

    pub fn cycle_item(&mut self) {
        self.item = match self.item {
            MenuItem::Rosary => MenuItem::EveningPrayer,
            MenuItem::EveningPrayer => MenuItem::Rosary,
            _ => self.item,
        }
    }
}

pub fn input_handler<'a>(
    rx: &Receiver<Event<KeyEvent>>,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    frame: &mut Frame,
) -> Result<MenuItem, Box<dyn Error>> {
    // Input handler
    match rx.recv()? {
        Event::Refresh(_, _) => {
            redraw(terminal, frame)?;
            Ok(frame.get_active_window().active_menu_item())
        }
        Event::Input(event) => match frame.get_active_window().active_menu_item() {
            MenuItem::Rosary => rosary_input_handler(&rx, terminal, frame, &event),
            MenuItem::EveningPrayer => evening_prayer_input_handler(&rx, terminal, frame, &event),
            _ => Ok(frame.get_active_window().active_menu_item()),
        },
        Event::Tick => Ok(frame.get_active_window().active_menu_item()),
    }
}

pub fn key_listen<'a>(
    rx: &'a Receiver<Event<KeyEvent>>,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    frame: &'a mut Frame,
) -> MenuItem {
    let new_menu_item = input_handler(&rx, terminal, frame);
    if new_menu_item.is_err() {
        frame
            .get_active_window()
            .set_error("Unable to read key.".to_owned());
        return frame.get_active_window().active_menu_item();
    }
    new_menu_item.unwrap()
}
