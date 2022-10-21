use crate::events::general_input_handler;
use crate::prayer::EveningPrayer;
use crate::rosary::Rosary;
use crate::{
    events::{evening_prayer_input_handler, rosary_input_handler},
    language::Language,
};
use crossterm::event::KeyEvent;
use soloud::Soloud;

use std::error::Error;
use std::fmt;
use std::io::Stdout;
use std::sync::mpsc::Receiver;
use tui::{backend::CrosstermBackend, Terminal};

use crate::language::Language::LATINA;
use crate::render::redraw;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MenuItem {
    _NOQUIT,
    Rosary,
    EveningPrayer,
    Settings,
    Quit,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            // Use _NOQUIT when calculating the true menu item is too costly,
            // but only the QUIT behavior is important
            MenuItem::_NOQUIT => 255,
            MenuItem::Quit => 0,
            MenuItem::Rosary => 1,
            MenuItem::Settings => 2,
            MenuItem::EveningPrayer => 3,
        }
    }
}

#[derive(Debug)]
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
    pub sl: Soloud,
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
        let mut sl = Soloud::default().expect("Error initializing audio");
        let mut w = Window::new();
        w.is_active = true;
        let ws = WindowStack::Node(w);
        Frame { ws, sl }
    }

    pub fn get_active_window(&mut self) -> &mut Window {
        return _get_active_window(&mut self.ws).unwrap();
    }

    pub fn vsplit(mut self) -> Frame {
        self.ws = WindowStack::VSplit(Box::from(self.ws), new_ws_box());
        self
    }

    pub fn hsplit(mut self) -> Frame {
        self.ws = WindowStack::HSplit(Box::from(self.ws), new_ws_box());
        self
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Popup {
    Audio,
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
    popup: Option<Popup>,
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
            popup: None,
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

    pub fn get_language(&self) -> &Language {
        &self.lang
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

#[derive(Debug)]
struct InvalidFocusError;

impl fmt::Display for InvalidFocusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pressed key in unknown window")
    }
}
impl Error for InvalidFocusError {}

pub fn input_handler<'a>(
    rx: &Receiver<Event<KeyEvent>>,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut frame: Frame,
) -> (Frame, Result<MenuItem, Box<dyn Error>>) {
    // Input handler
    let evnt = rx.recv();
    if evnt.is_err() {
        return (frame, Err(Box::new(InvalidFocusError)));
    }
    match rx.recv().unwrap() {
        Event::Refresh(_, _) => {
            let a = redraw(terminal, &mut frame);
            if a.is_err() {
                return (frame, Err(a.unwrap_err()));
            }
            (frame, Ok(MenuItem::_NOQUIT))
        }
        Event::Input(event) => {
            let (mut frame, gih) = general_input_handler(terminal, frame, &event);
            if gih.is_none() {
                let ami = frame.get_active_window().active_menu_item();
                match ami {
                    MenuItem::Rosary => {
                        let rih = rosary_input_handler(terminal, &mut frame, &event);
                        (frame, rih)
                    }
                    MenuItem::EveningPrayer => {
                        let epih = evening_prayer_input_handler(terminal, &mut frame, &event);
                        (frame, epih)
                    }
                    _ => (frame, Ok(ami)),
                }
            } else {
                (frame, gih.ok_or(Box::new(InvalidFocusError)))
            }
        }
        Event::Tick => (frame, Ok(MenuItem::_NOQUIT)),
    }
}

pub fn key_listen<'a>(
    rx: &'a Receiver<Event<KeyEvent>>,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    frame: Frame,
) -> (Frame, MenuItem) {
    let (mut frame, new_menu_item) = input_handler(&rx, terminal, frame);
    if new_menu_item.is_err() {
        frame
            .get_active_window()
            .set_error("Unable to read key.".to_owned());
        let ami = frame.get_active_window().active_menu_item();
        return (frame, ami);
    }
    (frame, new_menu_item.unwrap())
}
