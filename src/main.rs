use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event as CEvent},
    terminal::enable_raw_mode,
};
use rosarium::prayer::EveningPrayer;
use rosarium::render::redraw;
use rosarium::rosary::Rosary;
use rosarium::tui::{key_listen, Event, Frame, MenuItem, Window};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let mut frame = Frame::new();

    // Event loop
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    // Create terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            // Handle key events
            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can't read events") {
                    tx.send(Event::Input(key)).expect("can't send events");
                } else if let CEvent::Resize(x, y) = event::read().expect("can't read events") {
                    tx.send(Event::Refresh(x, y)).expect("can't send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    terminal.autoresize();
    redraw(&mut terminal, &mut frame)?;

    loop {
        let q = key_listen(&rx, &mut terminal, &mut frame);
        if q == MenuItem::Quit {
            break;
        }
    }
    Ok(())
}
