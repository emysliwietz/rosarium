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

pub fn get_keybindings(f: &Frame) -> String {
    let mut keybinds = String::from(
        "?:   Toggle Keybindings
r:   Refresh window
q:   Quit

j:   Scroll up
k:   Scroll down

x:   Cycle language
",
    );
    keybinds += match f.get_active_window_ro().active_menu_item() {
        MenuItem::Rosary => "\nSpace/l/Right: Advance Rosary\nBackspace/h/Left: Recede Rosary",
        MenuItem::PrayerSet(_) => {
            "\nSpace/l/Right: Advance Prayer\nBackspace/h/Left: Recede Prayer"
        }
        _ => "",
    };
    keybinds += "\n
p:   Play/Pause audio (if available for current window)
v:   Toggle Volume Popup
h:   Lower Volume (when volume popup active)
l:   Raise Volume (when volume popup active)
1-9: Set Volume to 10-90% (0 sets to 100%)

H:   Split window horizontal
L:   Split window vertical
";
    keybinds
}

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
        KeyCode::Char('v') => frame.toggle_volume_popup(),
        KeyCode::Char('?') => frame.toggle_keybinding_popup(),
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

pub fn volume_input_handler<'a>(
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    frame: &'a mut Frame,
    event: &KeyEvent,
) -> Result<Option<MenuItem>, Box<dyn Error>> {
    match event.code {
        KeyCode::Char('h') => frame.lower_volume(),
        KeyCode::Char('l') => frame.raise_volume(),
        KeyCode::Char('m') => frame.set_volume(0),
        KeyCode::Char('1') => frame.set_volume(10),
        KeyCode::Char('2') => frame.set_volume(20),
        KeyCode::Char('3') => frame.set_volume(30),
        KeyCode::Char('4') => frame.set_volume(40),
        KeyCode::Char('5') => frame.set_volume(50),
        KeyCode::Char('6') => frame.set_volume(60),
        KeyCode::Char('7') => frame.set_volume(70),
        KeyCode::Char('8') => frame.set_volume(80),
        KeyCode::Char('9') => frame.set_volume(90),
        KeyCode::Char('0') => frame.set_volume(100),
        _ => {}
    }
    redraw(terminal, frame)?;
    Ok(Some(frame.get_active_window_ro().active_menu_item()))
}

pub fn rosary_input_handler<'a>(
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
    frame: &'a mut Frame,
    event: &KeyEvent,
) -> Result<MenuItem, Box<dyn Error>> {
    match event.code {
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
