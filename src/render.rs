use crate::calender::weekday;
use crate::language::get_title_translation;
use crate::prayer::EveningPrayer;
use crate::rosary::{get_daily_mystery, Rosary};
use crate::tui::{Frame, MenuItem, Window, WindowStack};
use crate::tui_util::{center_p, cursive_p, hcenter};
use std::error::Error;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Text;
use tui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use tui::Terminal;

pub fn render_evening_prayer<'a>(window: &mut Window) -> Result<Paragraph<'a>, Box<dyn Error>> {
    let prayer = window.evening_prayer.to_prayer().clone();
    let prayer_render = cursive_p(
        prayer.get_prayer_text(window),
        "evening_prayer",
        prayer.get_prayer_title(window),
        window,
    );
    Ok(prayer_render)
}

pub fn render_prayer<'a>(window: &mut Window) -> Result<Paragraph<'a>, Box<dyn Error>> {
    let rosary_prayer = window.rosary.to_prayer();
    let mut prayer_words = rosary_prayer.get_prayer_text(window)?;
    let mut prayer_title = rosary_prayer.get_prayer_title(window);
    if rosary_prayer.is_mystery() {
        prayer_title = hcenter(&prayer_title, window);
        prayer_words = hcenter(&prayer_words, window);
    }
    let prayer_text = Text::from(prayer_words);
    let top_offset = window.get_top_offset(prayer_text.height() + 3);
    let mut centered_prayer_text: Text =
        Text::raw(String::from("\n") + &prayer_title + "\n" + &"\n".repeat(top_offset));
    let prayer_width = centered_prayer_text.width();
    if rosary_prayer.is_mystery() {
        centered_prayer_text.patch_style(
            Style::default()
                .remove_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::BOLD)
                .fg(rosary_prayer.to_color()),
        );
    } else {
        centered_prayer_text.patch_style(
            Style::default()
                .remove_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightYellow),
        );
    }
    centered_prayer_text.extend(prayer_text);

    let rosarium = Paragraph::new(centered_prayer_text)
        .wrap(Wrap { trim: false })
        .scroll(window.get_offset())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(
                    Style::default()
                        .fg(Color::White)
                        .remove_modifier(Modifier::ITALIC),
                )
                .title(get_title_translation("rosarium", window))
                .border_type(BorderType::Rounded),
        );
    Ok(if rosary_prayer.is_mystery() {
        let mut offset = window.get_offset();
        offset.0 = window.get_vert_offset(prayer_width) as u16;
        rosarium
            .style(
                Style::default()
                    .remove_modifier(Modifier::ITALIC)
                    .remove_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Left)
    } else {
        rosarium
            .style(
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .remove_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
    })
}

pub fn render_progress<'a>(window: &mut Window) -> Paragraph<'a> {
    let mut progress = Paragraph::new(if window.has_error() {
        window.error()
    } else {
        window.rosary.progress()
    })
    .alignment(Alignment::Right)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Oratio")
            .border_type(BorderType::Rounded),
    );
    if window.has_error() {
        progress = progress.style(Style::default().fg(Color::Red));
        window.clear_error();
    }
    progress
}

pub fn render_mysteries<'a>() -> Paragraph<'a> {
    let progress = Paragraph::new(get_daily_mystery())
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Mysteria Rosarii")
                .border_type(BorderType::Rounded),
        );
    progress
}

pub fn draw_rosary(
    window: &mut Window,
    rect: &mut tui::Frame<CrosstermBackend<Stdout>>,
    chunk: &mut Rect,
) -> Result<(), Box<dyn Error>> {
    let main_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Max(3)])
        .split(*chunk);
    let bottom_bar = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_split[1]);

    let prayer_window = render_prayer(window);
    if prayer_window.is_err() {
        window.set_error(prayer_window.as_ref().err().as_ref().unwrap().to_string());
    }
    rect.render_widget(prayer_window.unwrap(), main_split[0]);
    rect.render_widget(render_progress(window), bottom_bar[0]);
    rect.render_widget(render_mysteries(), bottom_bar[1]);

    window.set_parent_dims(chunk.width, chunk.height);
    Ok(())
}

pub fn draw_evening_prayer(
    window: &mut Window,
    rect: &mut tui::Frame<CrosstermBackend<Stdout>>,
    chunk: &mut Rect,
) -> Result<(), Box<dyn Error>> {
    let size = rect.size();
    let prayer_window = render_evening_prayer(window);
    if prayer_window.is_err() {
        window.set_error(prayer_window.as_ref().err().as_ref().unwrap().to_string());
    }
    rect.render_widget(prayer_window.unwrap(), *chunk);

    window.set_parent_dims(chunk.width, chunk.height);
    Ok(())
}

pub fn refresh(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    frame: &mut Frame,
) -> Result<(), Box<dyn Error>> {
    terminal.clear()?;
    redraw(terminal, frame)
}

pub fn redraw(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    frame: &mut Frame,
) -> Result<(), Box<dyn Error>> {
    terminal.draw(|rect| {
        let mut chunk: Rect = rect.size();
        redraw_recursive(&mut frame.ws, rect, &mut chunk);
    })?;
    // let chunks: Layout = Layout::default()
    //     .direction(Direction::Vertical)
    //    .margin(1)
    //   .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref());
    Ok(())
}

fn redraw_recursive(
    ws: &mut WindowStack,
    rect: &mut tui::Frame<CrosstermBackend<Stdout>>,
    chunk: &mut Rect,
) -> Result<(), Box<dyn Error>> {
    match ws {
        WindowStack::Node(w) => redraw_window(w, rect, chunk),
        WindowStack::HSplit(v, w) => {
            let mut hlayout = Layout::default()
                .margin(1)
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(*chunk);
            redraw_recursive(v, rect, &mut hlayout[0]);
            redraw_recursive(v, rect, &mut hlayout[1])
        }
        WindowStack::VSplit(v, w) => {
            let mut vlayout = Layout::default()
                .margin(1)
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(*chunk);
            redraw_recursive(v, rect, &mut vlayout[0]);
            redraw_recursive(v, rect, &mut vlayout[1])
        }
    }
}

fn redraw_window(
    window: &mut Window,
    rect: &mut tui::Frame<CrosstermBackend<Stdout>>,
    chunk: &mut Rect,
) -> Result<(), Box<dyn Error>> {
    match window.active_menu_item() {
        MenuItem::Rosary => draw_rosary(window, rect, chunk),
        MenuItem::Settings => {
            /*let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
            )
            .split(chunks[0]);*/
            Ok(())
        }
        MenuItem::Quit => Ok(()),
        MenuItem::_NOQUIT => Ok(()),
        MenuItem::EveningPrayer => draw_evening_prayer(window, rect, chunk),
    };
    Ok(())
}
