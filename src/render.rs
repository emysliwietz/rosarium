use crate::calender::AnnusLiturgicus;
use crate::events::get_keybindings;
use crate::language::get_title_translation;

use crate::rosary::get_daily_mystery;
use crate::tui::{Frame, MenuItem, Popup, Window, WindowStack};
use crate::tui_util::{centered_rect, cursive_p, hcenter};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Weekday};
use std::error::Error;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Text;
use tui::widgets::{Block, BorderType, Borders, Cell, Clear, Gauge, Paragraph, Row, Table, Wrap};
use tui::Terminal;

pub fn render_prayer_set<'a>(window: &mut Window) -> Result<Paragraph<'a>, Box<dyn Error>> {
    let language = window.get_language().clone();
    let prayer_set = window.get_curr_prayer_set()?;
    let prayer = prayer_set.to_prayer();
    let (title, text, audio) = prayer.title_text_audio(&language);
    let prayer_render = cursive_p(text, prayer_set.get_title(&language), title, window);
    window.audio = audio;
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
                .title(get_title_translation("rosarium", window.get_language()))
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

pub fn render_volume<'a>(frame: &mut Frame) -> Gauge<'a> {
    Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Volume")
                .border_type(BorderType::Rounded),
        )
        .gauge_style(Style::default().fg(Color::White).bg(Color::Black))
        .use_unicode(true)
        .ratio(frame.get_volume() as f64)
}

pub fn render_keybindings<'a>(frame: &mut Frame) -> Paragraph<'a> {
    Paragraph::new(get_keybindings(frame))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .scroll(frame.get_active_window().get_offset())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Keybindings")
                .border_type(BorderType::Rounded),
        )
}

pub fn render_error<'a>(frame: &mut Frame) -> Paragraph<'a> {
    Paragraph::new(frame.get_active_window().error())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .scroll(frame.get_active_window().get_offset())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .style(Style::default().fg(Color::Red))
                .title("Error")
                .border_type(BorderType::Rounded),
        )
}

pub fn render_calendar<'a>(
    al: &AnnusLiturgicus,
    _selected_day: DateTime<Local>,
    today: DateTime<Local>,
    window: &Window,
) -> Table<'a> {
    let mut items = vec![];
    let mut i = 0;
    let mut al = al.to_vec();
    if al[0].1.year() == today.year() {
        al.push(("Today", today.naive_local().date()));
    }
    al.sort_by(|a, b| a.1.cmp(&b.1));
    for (name, date) in al {
        if i >= window.get_offset().0 {
            items.push(Row::new(vec![date.to_string(), name.to_owned()]));
        }
        i += 1;
    }
    Table::new(items)
        .block(
            Block::default()
                .title("Calendar")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .header(Row::new(vec!["Date".to_owned(), "Name".to_owned()]).bottom_margin(1))
        .widths(&[Constraint::Min(12), Constraint::Min(30)])
}

pub fn render_month<'a>(
    _al: &AnnusLiturgicus,
    selected_day: DateTime<Local>,
    today: DateTime<Local>,
    _window: &Window,
) -> Result<Table<'a>, Box<dyn Error>> {
    let mut day = NaiveDate::from_ymd_opt(selected_day.year(), selected_day.month(), 1)
        .ok_or("Date could not be parsed")?;
    let a = day.weekday().num_days_from_sunday();
    let mut weeks = vec![];
    let mut week_row = vec![Cell::from(" ")];
    for _ in 0..a {
        week_row.push(Cell::from("  "))
    }
    while day.month() == selected_day.month() {
        let mut d = Cell::from(format!("{:0>2}", day.day()));
        if day == selected_day.naive_local().date() {
            d = d.style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::LightYellow)
                    .add_modifier(Modifier::ITALIC)
                    .add_modifier(Modifier::BOLD),
            );
        } else if day == today.naive_local().date() {
            d = d.style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC)
                    .add_modifier(Modifier::BOLD),
            );
        }
        week_row.push(d);
        if day.weekday() == Weekday::Sat {
            week_row.push(Cell::from(""));
            weeks.push(Row::new(week_row));
            week_row = vec![Cell::from(" ")];
        }
        day = day.succ_opt().ok_or("No succeding day to {day}")?;
    }
    weeks.push(Row::new(week_row));
    Ok(Table::new(weeks)
        .block(
            Block::default()
                .title(format!("{} {}", selected_day.month(), selected_day.year()))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .header(Row::new(vec!["", "Su", "Mo", "Tu", "We", "Th", "Fr", "Sa", ""]).bottom_margin(1))
        .widths(&[
            Constraint::Max(1),
            Constraint::Min(2),
            Constraint::Min(2),
            Constraint::Min(2),
            Constraint::Min(2),
            Constraint::Min(2),
            Constraint::Min(2),
            Constraint::Min(2),
            Constraint::Min(0),
        ]))
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

pub fn draw_prayer_set(
    window: &mut Window,
    rect: &mut tui::Frame<CrosstermBackend<Stdout>>,
    chunk: &mut Rect,
) -> Result<(), Box<dyn Error>> {
    let prayer_window = render_prayer_set(window);
    if prayer_window.is_err() {
        window.set_error(prayer_window.as_ref().err().as_ref().unwrap().to_string());
    }
    rect.render_widget(prayer_window.unwrap(), *chunk);

    window.set_parent_dims(chunk.width, chunk.height);
    Ok(())
}

pub fn draw_frame_popup(
    frame: &mut Frame,
    rect: &mut tui::Frame<CrosstermBackend<Stdout>>,
    chunk: &mut Rect,
) {
    if frame.get_popup().is_none() {
        return;
    }
    match frame.get_popup().unwrap() {
        &Popup::Volume => draw_volume_popup(frame, rect, chunk),
        &Popup::KeyBindings => draw_keybinding_popup(frame, rect, chunk),
        &Popup::Error => draw_error_popup(frame, rect, chunk),
    }
}

pub fn draw_keybinding_popup(
    frame: &mut Frame,
    rect: &mut tui::Frame<CrosstermBackend<Stdout>>,
    chunk: &mut Rect,
) {
    let popup_chunk = centered_rect(80, 60, 10, chunk);
    rect.render_widget(Clear, popup_chunk);
    rect.render_widget(render_keybindings(frame), popup_chunk);
}

pub fn draw_error_popup(
    frame: &mut Frame,
    rect: &mut tui::Frame<CrosstermBackend<Stdout>>,
    chunk: &mut Rect,
) {
    let popup_chunk = centered_rect(80, 60, 10, chunk);
    rect.render_widget(Clear, popup_chunk);
    rect.render_widget(render_error(frame), popup_chunk);
}

pub fn draw_volume_popup(
    frame: &mut Frame,
    rect: &mut tui::Frame<CrosstermBackend<Stdout>>,
    chunk: &mut Rect,
) {
    let popup_chunk = centered_rect(80, 1, 3, chunk);
    rect.render_widget(Clear, popup_chunk);
    rect.render_widget(render_volume(frame), popup_chunk);
}

pub fn draw_calendar(
    window: &mut Window,
    rect: &mut tui::Frame<CrosstermBackend<Stdout>>,
    chunk: &mut Rect,
) -> Result<(), Box<dyn Error>> {
    let split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Max(24)])
        .split(*chunk);
    let today = chrono::offset::Local::now();
    let day_offset = window.get_signed_offset().1;
    let selected_day = today
        .checked_add_signed(Duration::days(day_offset.into()))
        .unwrap();
    let al = AnnusLiturgicus::new(selected_day.year())?;
    rect.render_widget(render_calendar(&al, selected_day, today, window), split[0]);
    rect.render_widget(render_month(&al, selected_day, today, window)?, split[1]);
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
        //TODO Handle main error
        let e = redraw_recursive(&mut frame.ws, rect, &mut chunk);
        if e.is_err() {
            frame
                .get_active_window()
                .set_error(e.unwrap_err().to_string());
            draw_error_popup(frame, rect, &mut chunk);
        } else {
            draw_frame_popup(frame, rect, &mut chunk);
        }
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
            redraw_recursive(v, rect, &mut hlayout[0])?;
            redraw_recursive(w, rect, &mut hlayout[1])
        }
        WindowStack::VSplit(v, w) => {
            let mut vlayout = Layout::default()
                .margin(1)
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(*chunk);
            redraw_recursive(v, rect, &mut vlayout[0])?;
            redraw_recursive(w, rect, &mut vlayout[1])
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
        MenuItem::Calendar => draw_calendar(window, rect, chunk),
        MenuItem::Settings => Ok(()),
        MenuItem::Quit => Ok(()),
        MenuItem::_NOQUIT => Ok(()),
        MenuItem::PrayerSet(_) => draw_prayer_set(window, rect, chunk),
    }?;
    Ok(())
}
