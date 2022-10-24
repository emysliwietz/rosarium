use crate::{
    render::{redraw, refresh},
    tui::{Event, Frame, MenuItem},
};
use crossterm::event::KeyEvent;
use crossterm::{event::KeyCode, terminal::disable_raw_mode};
use std::error::Error;
use std::io::Stdout;
use std::sync::mpsc::Receiver;
use tui::{backend::CrosstermBackend, Terminal};

pub fn general_input_handler<'a>(
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    mut frame: Frame,
    event: &KeyEvent,
) -> (Frame, Option<MenuItem>) {
    match event.code {
        KeyCode::Char('q') => {
            let a = disable_raw_mode();
            let b = terminal.show_cursor();
            if a.is_err() || b.is_err() {
                return (frame, None);
            }
            return (frame, Some(MenuItem::Quit));
        }
        KeyCode::Char('r') => {
            let a = refresh(terminal, &mut frame);
            if a.is_err() {
                return (frame, None);
            }
        }

        KeyCode::Tab => frame.get_active_window().cycle_item(),
        KeyCode::Char('k') => frame.get_active_window().down(),
        KeyCode::Char('j') => frame.get_active_window().up(),
        KeyCode::Char('x') => frame.get_active_window().cycle_language(),
        KeyCode::Char('H') => frame = frame.hsplit(),
        KeyCode::Char('L') => frame = frame.vsplit(),
        KeyCode::Char('p') => {
            frame.toggle_audio();
        }
        _ => return (frame, None),
    }
    let a = redraw(terminal, &mut frame);
    if a.is_err() {
        return (frame, None);
    }
    let active_menu_item = frame.get_active_window().active_menu_item();
    (frame, Some(active_menu_item))
}

pub fn rosary_input_handler<'a>(
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
        KeyCode::Char(' ') => frame.get_active_window().rosary.advance(),
        KeyCode::Char('l') => frame.get_active_window().rosary.advance(),
        KeyCode::Char('h') => frame.get_active_window().rosary.recede(),
        KeyCode::Left => frame.get_active_window().rosary.recede(),
        KeyCode::Right => frame.get_active_window().rosary.advance(),
        KeyCode::Backspace => frame.get_active_window().rosary.recede(),
        _ => {}
    }
    redraw(terminal, frame)?;
    Ok(frame.get_active_window().active_menu_item())
}

pub fn prayer_set_input_handler<'a>(
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
        KeyCode::Char(' ') => frame.get_active_window().get_curr_prayer_set().advance(),
        KeyCode::Char('l') => frame.get_active_window().get_curr_prayer_set().advance(),
        KeyCode::Char('h') => frame.get_active_window().get_curr_prayer_set().recede(),
        KeyCode::Left => frame.get_active_window().get_curr_prayer_set().recede(),
        KeyCode::Right => frame.get_active_window().get_curr_prayer_set().advance(),
        KeyCode::Backspace => frame.get_active_window().get_curr_prayer_set().recede(),
        _ => {}
    }
    redraw(terminal, frame)?;
    Ok(frame.get_active_window().active_menu_item())
}
