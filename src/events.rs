use crate::{
    prayer::EveningPrayer,
    render::{redraw, refresh},
    rosary::Rosary,
    tui::{Event, MenuItem, Window},
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
    rosary: &'a mut Rosary,
    evening_prayer: &'a mut EveningPrayer,
    window: &'a mut Window,
    event: &KeyEvent,
) -> Result<MenuItem, Box<dyn Error>> {
    match event.code {
        KeyCode::Char('q') => {
            disable_raw_mode()?;
            terminal.show_cursor()?;
            return Ok(MenuItem::Quit);
        }
        KeyCode::Char('r') => refresh(terminal, &rosary, evening_prayer, window)?,
        KeyCode::Char(' ') => rosary.advance(),
        KeyCode::Char('l') => rosary.advance(),
        KeyCode::Char('h') => rosary.recede(),
        KeyCode::Char('k') => window.down(),
        KeyCode::Char('j') => window.up(),
        KeyCode::Char('x') => window.cycle_language(),
        KeyCode::Char('H') => window.left(),
        KeyCode::Char('L') => window.right(),
        KeyCode::Right => rosary.advance(),
        KeyCode::Backspace => rosary.recede(),
        KeyCode::Left => rosary.recede(),
        KeyCode::Tab => window.cycle_item(),
        _ => {}
    }
    redraw(terminal, &rosary, evening_prayer, window)?;
    Ok(window.active_menu_item())
}

pub fn evening_prayer_input_handler<'a>(
    rx: &Receiver<Event<KeyEvent>>,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    rosary: &'a mut Rosary,
    evening_prayer: &'a mut EveningPrayer,
    window: &'a mut Window,
    event: &KeyEvent,
) -> Result<MenuItem, Box<dyn Error>> {
    match event.code {
        KeyCode::Char('q') => {
            disable_raw_mode()?;
            terminal.show_cursor()?;
            return Ok(MenuItem::Quit);
        }
        KeyCode::Char('r') => refresh(terminal, &rosary, evening_prayer, window)?,
        KeyCode::Char(' ') => evening_prayer.advance(),
        KeyCode::Char('l') => evening_prayer.advance(),
        KeyCode::Char('h') => evening_prayer.recede(),
        KeyCode::Right => evening_prayer.advance(),
        KeyCode::Backspace => evening_prayer.recede(),
        KeyCode::Left => evening_prayer.recede(),
        KeyCode::Char('k') => window.down(),
        KeyCode::Char('j') => window.up(),
        KeyCode::Char('x') => window.cycle_language(),
        KeyCode::Char('H') => window.left(),
        KeyCode::Char('L') => window.right(),
        KeyCode::Tab => window.cycle_item(),
        _ => {}
    }
    redraw(terminal, &rosary, evening_prayer, window)?;
    Ok(window.active_menu_item())
}
