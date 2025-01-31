use std::io::{self, stdout};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use crossterm::terminal::disable_raw_mode;
use crossterm::ExecutableCommand;
use crossterm::{
    event::{self, Event as CEvent},
    terminal::enable_raw_mode,
};

use ratatui::{backend::CrosstermBackend, Terminal};
use rosarium::render::redraw;
use rosarium::tui::{key_listen, Event, Frame, MenuItem};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Duration;
    stdout()
        .execute(event::EnableMouseCapture)
        .expect("No mouse capture support");
    enable_raw_mode().expect("can not run in raw mode");

    let mut frame = Frame::new()?;

    // Event loop
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    // Create terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    thread::Builder::new()
        .name("rosarium - keylistener".to_string())
        .spawn(move || {
            let last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(200));

                // Handle key events
                if event::poll(timeout).expect("poll works") {
                    if let CEvent::Key(key) = event::read().expect("can't read events") {
                        tx.send(Event::Input(key)).expect("can't send events");
                        tx.send(Event::Input(key)).expect("can't send events");
                        // println!("{:?}", Event::Input(key));
                    } else if let CEvent::Resize(x, y) = event::read().expect("can't read events") {
                        tx.send(Event::Refresh(x, y)).expect("can't send events");
                    } else if let CEvent::Mouse(_m) = event::read().expect("can't read events") {
                        //println!("{:?}", m);
                    }
                }
                /*
                if last_tick.elapsed() >= tick_rate {
                    if let Ok(_) = tx.send(Event::Tick) {
                        last_tick = Instant::now();
                    }
                }*/
            }
        })?;

    redraw(&mut terminal, &mut frame)?;
    loop {
        let (f, q) = key_listen(&rx, &mut terminal, frame);
        frame = f;
        //implement error handling
        if q == MenuItem::Quit {
            std::io::stdout()
                .execute(event::DisableMouseCapture)
                .expect("No mouse capture support");
            disable_raw_mode().expect("can not run in raw mode");
            break;
        }
    }
    Ok(())
}
