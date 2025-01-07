use crate::audio::{audio_thread, AudioCommand};
use crate::config_parse::get_all_prayset_titles;
use crate::events::{
    calendar_input_handler, general_input_handler, prayer_set_input_handler, volume_input_handler,
};
use crate::prayer::PrayerSet;
use crate::rosary::Rosary;
use crate::{events::rosary_input_handler, language::Language};
use chrono::Datelike;
use crossterm::event::KeyEvent;
use rand::rngs::StdRng;
use rand::SeedableRng;
use soloud::{AudioExt, LoadExt, Soloud, Speech, Wav, WavStream};

use std::borrow::Borrow;
use std::error::Error;
use std::fmt;
use std::io::Stdout;
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use tui::{backend::CrosstermBackend, Terminal};

use crate::language::Language::LATINA;
use crate::render::redraw;

#[derive(Debug)]
pub enum ErrorString {
    Error(&'static str),
}
pub type E = Box<dyn Error>;

pub fn e(s: &'static str) -> E {
    Box::new(ErrorString::Error(s))
}

impl std::fmt::Display for ErrorString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorString::Error(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for ErrorString {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MenuItem {
    _NOQUIT,
    Rosary,
    PrayerSet(usize),
    Calendar,
    Settings,
    Quit,
}

#[derive(Debug)]
pub enum Event<I> {
    Input(I),
    Refresh(u16, u16),
    Tick,
}

#[derive(Debug)]
pub enum WindowStack {
    HSplit(Box<WindowStack>, Box<WindowStack>),
    VSplit(Box<WindowStack>, Box<WindowStack>),
    Node(Window),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Popup {
    Volume,
    KeyBindings,
    Error,
}

#[derive(Debug)]
pub struct Frame {
    pub ws: WindowStack,
    tx: Sender<AudioCommand>,
    popup: Option<Popup>,
    volume: f32,
}

fn new_ws_box() -> Result<Box<WindowStack>, E> {
    Ok(Box::from(WindowStack::Node(Window::new()?)))
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

pub fn _get_active_window_read_only(s: &WindowStack) -> Option<&Window> {
    match s {
        WindowStack::Node(w) => {
            if w.is_active {
                return Some(&w);
            }
        }
        WindowStack::HSplit(v, w) => {
            let v = _get_active_window_read_only(v);
            let w = _get_active_window_read_only(w);
            if v.is_some() {
                return v;
            } else if w.is_some() {
                return w;
            }
        }
        WindowStack::VSplit(v, w) => {
            let v = _get_active_window_read_only(v);
            let w = _get_active_window_read_only(w);
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
    pub fn new() -> Result<Frame, E> {
        let mut w = Window::new()?;
        w.is_active = true;
        let ws = WindowStack::Node(w);
        let (tx, rx) = mpsc::channel();
        audio_thread(rx);
        Ok(Frame {
            ws,
            tx,
            popup: None,
            volume: 1.0,
        })
    }

    pub fn get_popup(&self) -> Option<&Popup> {
        self.popup.as_ref()
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn lower_volume(&mut self) {
        if self.volume >= 0.01 {
            self.volume -= 0.01;
            self.tx.send(AudioCommand::SetVolume(self.volume));
        }
    }

    pub fn raise_volume(&mut self) {
        if self.volume < 1.00 {
            self.volume += 0.01;
            self.tx.send(AudioCommand::SetVolume(self.volume));
        }
    }

    pub fn set_volume(&mut self, percentage: u8) {
        self.volume = percentage as f32 / 100.0;
        self.tx.send(AudioCommand::SetVolume(self.volume));
    }

    fn toggle_popup(&mut self, p: Popup) {
        if self.popup.is_none() {
            self.popup = Some(p)
        } else {
            self.popup = None
        }
    }

    pub fn toggle_volume_popup(&mut self) {
        self.toggle_popup(Popup::Volume)
    }

    pub fn toggle_keybinding_popup(&mut self) {
        self.toggle_popup(Popup::KeyBindings)
    }

    pub fn get_active_window(&mut self) -> &mut Window {
        return _get_active_window(&mut self.ws).unwrap();
    }

    pub fn get_active_window_ro(&self) -> &Window {
        return _get_active_window_read_only(&self.ws).unwrap();
    }

    pub fn vsplit(mut self) -> (Frame, Result<(), E>) {
        let ws_box = new_ws_box();
        if ws_box.is_err() {
            return (self, Err(ws_box.unwrap_err()));
        }
        let ws_box = ws_box.unwrap();
        self.ws = WindowStack::VSplit(Box::from(self.ws), ws_box);
        (self, Ok(()))
    }

    pub fn hsplit(mut self) -> (Frame, Result<(), E>) {
        let ws_box = new_ws_box();
        if ws_box.is_err() {
            return (self, Err(ws_box.unwrap_err()));
        }
        let ws_box = ws_box.unwrap();
        self.ws = WindowStack::HSplit(Box::from(self.ws), ws_box);
        (self, Ok(()))
    }

    pub fn set_error(&mut self, error: String) {
        self.popup = Some(Popup::Error);
        self.get_active_window().last_error = error;
    }

    pub fn check_error(&mut self) {
        let le = self.get_active_window().last_error.clone();
        if le != "" {
            self.set_error(le);
        }
    }

    pub fn toggle_audio(&mut self) {
        let caw = self.get_active_window();
        if caw.audio.is_some() {
            let audio = caw.audio.as_ref().unwrap().to_owned();
            caw.is_playing = true;
            self.tx.send(AudioCommand::Play(audio));
        } else {
            caw.is_playing = false;
            self.tx.send(AudioCommand::Pause);
        }
    }
}

#[derive(Debug)]
pub struct Window {
    x: i16,
    y: i16,
    lang: Language,
    parent_h: u16,
    parent_w: u16,
    pub last_error: String,
    item: MenuItem,
    is_playing: bool,
    pub audio: Option<String>,
    pub is_active: bool,
    pub rosary: Rosary,
    pub prayersets: Vec<PrayerSet>,
    rng: StdRng,
}

impl Window {
    pub fn new() -> Result<Window, E> {
        let today = chrono::offset::Local::now()
            .date()
            .naive_local()
            .num_days_from_ce() as u64;
        let mut rng = StdRng::seed_from_u64(today);

        let mut prayersets = vec![];
        for (title, yaml) in get_all_prayset_titles()? {
            prayersets.push(PrayerSet::new(title, yaml, &mut rng)?)
        }
        Ok(Window {
            x: 0,
            y: 0,
            lang: LATINA,
            parent_h: 0,
            parent_w: 0,
            last_error: String::from(""),
            item: MenuItem::Rosary,
            is_active: false,
            is_playing: false,
            audio: None,
            rosary: Rosary::new(),
            prayersets,
            rng,
        })
    }
    pub fn active_menu_item(&self) -> MenuItem {
        return self.item;
    }

    pub fn get_offset(&self) -> (u16, u16) {
        let x = if self.x < 0 { 0 } else { self.x };
        let y = if self.y < 0 { 0 } else { self.y };
        (x as u16, y as u16)
    }

    pub fn get_signed_offset(&self) -> (i16, i16) {
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

    pub fn set_error(&mut self, error: String) {
        self.last_error = error;
    }

    pub fn down(&mut self) {
        if self.x != 0 {
            self.x -= 1;
        }
    }

    pub fn up(&mut self) {
        if self.x != i16::MAX {
            self.x += 1;
        }
    }

    pub fn left(&mut self) {
        if self.y != i16::MIN {
            self.y -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.y != i16::MAX {
            self.y += 1;
        }
    }

    pub fn reset_horizontal_scroll(&mut self) {
        self.y = 0
    }

    pub fn set_parent_dims(&mut self, w: u16, h: u16) {
        self.parent_w = w;
        self.parent_h = h;
    }

    pub fn get_y(&self) -> i16 {
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

    pub fn cycle_language(&mut self) {
        match self.lang {
            Language::GERMANA => {
                self.lang = Language::LATINA;
            }
            Language::LATINA => {
                self.lang = Language::ANGLIA;
            }
            Language::ANGLIA => {
                self.lang = Language::SLAVONICA;
            }
            Language::SLAVONICA => {
                self.lang = Language::GERMANA;
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
        let num_prayer_sets = self.prayersets.len();
        self.item = match self.item {
            MenuItem::Rosary => {
                if num_prayer_sets > 0 {
                    MenuItem::PrayerSet(0)
                } else {
                    MenuItem::Calendar
                }
            }
            MenuItem::PrayerSet(i) => {
                if i < num_prayer_sets - 1 {
                    MenuItem::PrayerSet(i + 1)
                } else {
                    MenuItem::Calendar
                }
            }
            MenuItem::Calendar => MenuItem::Rosary,
            _ => self.item,
        }
    }

    pub fn get_curr_prayer_set_index(&self) -> Option<usize> {
        match self.item {
            MenuItem::PrayerSet(i) => Some(i),
            _ => None,
        }
    }

    pub fn get_curr_prayer_set(&mut self) -> Result<&mut PrayerSet, E> {
        let i = self.get_curr_prayer_set_index().unwrap_or(0);
        self.prayersets
            .get_mut(i)
            .ok_or(e("Can't find prayerset at current index"))
    }
}

#[derive(Debug)]
struct InvalidFocusError;

impl fmt::Display for InvalidFocusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pressed key in unknown window")
    }
}

pub fn popup_input_handler(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut frame: &mut Frame,
    event: &KeyEvent,
) -> Result<Option<MenuItem>, Box<dyn Error>> {
    let mut popup = frame.get_popup();
    if popup.is_none() {
        return Ok(None);
    }
    let popup = popup.unwrap();
    match popup {
        &Popup::Volume => return volume_input_handler(terminal, frame, event),
        &Popup::KeyBindings => return Ok(None),
        &Popup::Error => return Ok(None),
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
            if gih.is_err() {
                return (frame, Err(gih.unwrap_err()));
            }
            let gih = gih.unwrap();
            if gih == MenuItem::_NOQUIT {
                let popup_new_menu_item = popup_input_handler(terminal, &mut frame, &event);
                if popup_new_menu_item.is_err() {
                    return (frame, Err(popup_new_menu_item.unwrap_err()));
                } else {
                    let popup_new_menu_item = popup_new_menu_item.unwrap();
                    if popup_new_menu_item.is_some() {
                        return (frame, Ok(popup_new_menu_item.unwrap()));
                    }
                }
                let ami = frame.get_active_window().active_menu_item();
                match ami {
                    MenuItem::Rosary => {
                        let rih = rosary_input_handler(terminal, &mut frame, &event);
                        (frame, rih)
                    }
                    MenuItem::PrayerSet(_) => {
                        let epih = prayer_set_input_handler(terminal, &mut frame, &event);
                        (frame, epih)
                    }
                    MenuItem::Calendar => {
                        let epih = calendar_input_handler(terminal, &mut frame, &event);
                        (frame, epih)
                    }

                    _ => (frame, Ok(ami)),
                }
            } else {
                (frame, Ok(gih))
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
        frame.set_error(new_menu_item.unwrap_err().to_string());
        let ami = frame.get_active_window().active_menu_item();
        return (frame, ami);
    }
    (frame, new_menu_item.unwrap())
}
