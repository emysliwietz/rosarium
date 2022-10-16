use crate::{
    prayer::EveningPrayer,
    render::{redraw, refresh},
    rosary::Rosary,
    tui::{Event, Frame, MenuItem, Window},
};
use crossterm::event::KeyEvent;
use crossterm::{
    event::{self, KeyCode},
    terminal::disable_raw_mode,
};
use std::error::Error;
use std::io::Stdout;
use std::sync::mpsc::Receiver;
use tui::{backend::CrosstermBackend, Terminal};

pub fn rosary_input_handler<'a>(
    rx: &Receiver<Event<KeyEvent>>,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    frame: &'a mut Frame,
    event: &KeyEvent,
) -> Result<MenuItem, Box<dyn Error>> {
    match event.code {
        KeyCode::Char('q') => {
            disable_raw_mode()?;
            terminal.show_cursor()?;
            return Ok(MenuItem::Quit);
        }
        KeyCode::Char('r') => refresh(terminal, frame)?,
        KeyCode::Char(' ') => frame.get_active_window().rosary.advance(),
        KeyCode::Char('l') => frame.get_active_window().rosary.advance(),
        KeyCode::Char('h') => frame.get_active_window().rosary.recede(),
        KeyCode::Char('k') => frame.get_active_window().down(),
        KeyCode::Char('j') => frame.get_active_window().up(),
        KeyCode::Char('x') => frame.get_active_window().cycle_language(),
        KeyCode::Char('H') => frame.get_active_window().left(),
        KeyCode::Char('L') => frame.get_active_window().right(),
        KeyCode::Right => frame.get_active_window().rosary.advance(),
        KeyCode::Backspace => frame.get_active_window().rosary.recede(),
        KeyCode::Left => frame.get_active_window().rosary.recede(),
        KeyCode::Tab => frame.get_active_window().cycle_item(),
        _ => {}
    }
    redraw(terminal, frame)?;
    Ok(frame.get_active_window().active_menu_item())
}

pub fn evening_prayer_input_handler<'a>(
    rx: &Receiver<Event<KeyEvent>>,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    frame: &'a mut Frame,
    event: &KeyEvent,
) -> Result<MenuItem, Box<dyn Error>> {
    match event.code {
        KeyCode::Char('q') => {
            disable_raw_mode()?;
            terminal.show_cursor()?;
            return Ok(MenuItem::Quit);
        }
        KeyCode::Char('r') => refresh(terminal, frame)?,
        KeyCode::Char(' ') => frame.get_active_window().evening_prayer.advance(),
        KeyCode::Char('l') => frame.get_active_window().evening_prayer.advance(),
        KeyCode::Char('h') => frame.get_active_window().evening_prayer.recede(),
        KeyCode::Right => frame.get_active_window().evening_prayer.advance(),
        KeyCode::Backspace => frame.get_active_window().evening_prayer.recede(),
        KeyCode::Left => frame.get_active_window().evening_prayer.recede(),
        KeyCode::Char('k') => frame.get_active_window().down(),
        KeyCode::Char('j') => frame.get_active_window().up(),
        KeyCode::Char('x') => frame.get_active_window().cycle_language(),
        KeyCode::Char('H') => frame.get_active_window().left(),
        KeyCode::Char('L') => frame.get_active_window().right(),
        KeyCode::Tab => frame.get_active_window().cycle_item(),
        _ => {}
    }
    redraw(terminal, frame)?;
    Ok(frame.get_active_window().active_menu_item())
}
