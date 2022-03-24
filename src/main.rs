use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::{distributions::Alphanumeric, prelude::*};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};
use tui::layout::Constraint::Percentage;
use tui::symbols::line::{CROSS, THICK_CROSS};
use tui::text::Text;
use tui::widgets::Wrap;
use rosarium::rosary::{get_daily_mystery, Rosary};
use rosarium::tui::{Window};

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Rosary,
    Settings,
}


enum Event<I> {
    Input(I),
    Tick,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Rosary => 0,
            MenuItem::Settings => 1,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let mut rosary = Rosary::new();
    let mut window = Window::new();

    // Event loop
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // Create terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Handle tabs
    let menu_titles = vec!["Rosarium", "Settings", "Quit"];
    let mut active_menu_item = MenuItem::Rosary;

    loop {
        terminal.draw(|rect| {
            // Window layout
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                        .as_ref(),
                )
                .split(size);

            let bottom_bar = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),)
                .split(chunks[1]);

            rect.render_widget(render_progress(&rosary), bottom_bar[0]);
            rect.render_widget(render_mysteries(), bottom_bar[1]);

            window.set_parent_dims(chunks[0].width, chunks[0].height);

            // render current tab
            match active_menu_item {
                MenuItem::Rosary => rect.render_widget(render_prayer(&rosary, &window), chunks[0]),
                MenuItem::Settings => {
                    let pets_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[0]);
                }
            }
        })?;

        // Input handler
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('r') => active_menu_item = MenuItem::Rosary,
                KeyCode::Char('s') => active_menu_item = MenuItem::Settings,
                KeyCode::Char(' ') => advance(&mut rosary),
                KeyCode::Char('l') => advance(&mut rosary),
                KeyCode::Char('h') => recede(&mut rosary),
                KeyCode::Char('j') => scroll_down(&mut window, &rosary),
                KeyCode::Char('k') => scroll_up(&mut window, &rosary),
                KeyCode::Right => advance(&mut rosary),
                KeyCode::Backspace => recede(&mut rosary),
                KeyCode::Left => recede(&mut rosary),
                KeyCode::Up => {}
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

fn advance(rosary: &mut Rosary) {
    rosary.advance();
    render_progress(&rosary);
}

fn recede(rosary: &mut Rosary) {
    rosary.recede();
    render_progress(&rosary);
}

fn scroll_down(window: &mut Window, rosary: &Rosary) {
    window.down();
    render_prayer(rosary, window);
}

fn scroll_up(window: &mut Window, rosary: &Rosary) {
    window.up();
    render_prayer(rosary, window);
}

fn render_prayer<'a>(rosary: &Rosary, window: &Window) -> Paragraph<'a> {
    let rosary_prayer = rosary.to_prayer();
    let prayer_text = Text::from(rosary_prayer.get_prayer_text());
    let prayer_title = rosary_prayer.get_prayer_title();
    let top_offset = window.get_top_offset(prayer_text.height() + 3);
    let mut centered_prayer_text = Text::raw(String::from("\n") + &prayer_title + "\n" + &"\n".repeat(top_offset));
    centered_prayer_text.patch_style(Style::default().remove_modifier(Modifier::ITALIC).add_modifier(Modifier::BOLD).fg(Color::LightYellow));
    centered_prayer_text.extend(prayer_text);
    let rosarium = Paragraph::new(
        centered_prayer_text
    )
        .style(Style::default().add_modifier(Modifier::ITALIC).remove_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .scroll(window.get_offset())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White).remove_modifier(Modifier::ITALIC))
                .title("Rosarium")
                .border_type(BorderType::Rounded),
        );
    rosarium
}

fn render_progress<'a>(rosary: &Rosary) -> Paragraph<'a> {
    let progress = Paragraph::new(rosary.progress())
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Oratio")
                .border_type(BorderType::Rounded)
        );
    progress
}

fn render_mysteries<'a>() -> Paragraph<'a> {
    let progress = Paragraph::new(get_daily_mystery())
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Mysteria Rosarii")
                .border_type(BorderType::Rounded)
        );
    progress
}

/*
fn render_settings<'a>(pet_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let pets = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Pets")
        .border_type(BorderType::Plain);

    let list = List::new(items).block(pets).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let pet_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_pet.id.to_string())),
        Cell::from(Span::raw(selected_pet.name)),
        Cell::from(Span::raw(selected_pet.category)),
        Cell::from(Span::raw(selected_pet.age.to_string())),
        Cell::from(Span::raw(selected_pet.created_at.to_string())),
    ])])
        .header(Row::new(vec![
            Cell::from(Span::styled(
                "ID",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Name",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Category",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Age",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Created At",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Detail")
                .border_type(BorderType::Plain),
        )
        .widths(&[
            Constraint::Percentage(5),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(5),
            Constraint::Percentage(20),
        ]);

    (list, pet_detail)
}
*/