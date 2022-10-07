use std::error::Error;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::Terminal;
use tui::text::Text;
use tui::widgets::{Block, Borders, BorderType, Paragraph, Wrap};
use crate::calender::weekday;
use crate::language::get_title_translation;
use crate::rosary::{get_daily_mystery, Rosary};
use crate::tui::{center, Window, MenuItem};

pub fn render_evening_prayer<'a>(window: &mut Window) -> Result<Paragraph<'a>, Box<dyn Error>> {
    let prayer_text = Text::from(weekday());

    let evening_prayer = Paragraph::new(
        prayer_text
    )
        .wrap(Wrap { trim: false })
        .scroll(window.get_offset())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White).remove_modifier(Modifier::ITALIC))
                .title(get_title_translation("evening_prayer", window))
                .border_type(BorderType::Rounded),
        );
    Ok(evening_prayer)
}

pub fn render_prayer<'a>(rosary: &Rosary, window: &mut Window) -> Result<Paragraph<'a>, Box<dyn Error>> {
    let rosary_prayer = rosary.to_prayer();
    let mut prayer_words = rosary_prayer.get_prayer_text(rosary, window)?;
    let mut prayer_title = rosary_prayer.get_prayer_title(window);
    if rosary_prayer.is_mystery() {
        prayer_title = center(&prayer_title, window);
        prayer_words = center(&prayer_words, &window);
    }
    let prayer_text = Text::from(prayer_words);
    let top_offset = window.get_top_offset(prayer_text.height() + 3);
    let mut centered_prayer_text = Text::raw(String::from("\n") + &prayer_title + "\n" + &"\n".repeat(top_offset));
    let prayer_width = centered_prayer_text.width();
    if rosary_prayer.is_mystery() {
        centered_prayer_text.patch_style(Style::default().remove_modifier(Modifier::ITALIC).add_modifier(Modifier::BOLD).fg(rosary_prayer.to_color()));
    } else {
        centered_prayer_text.patch_style(Style::default().remove_modifier(Modifier::ITALIC).add_modifier(Modifier::BOLD).fg(Color::LightYellow));
    }
    centered_prayer_text.extend(prayer_text);

    let rosarium = Paragraph::new(
        centered_prayer_text
    )
        .wrap(Wrap { trim: false })
        .scroll(window.get_offset())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White).remove_modifier(Modifier::ITALIC))
                .title(get_title_translation("rosarium", window))
                .border_type(BorderType::Rounded),
        );
    Ok(if rosary_prayer.is_mystery() {
        let mut offset = window.get_offset();
        offset.0 = window.get_vert_offset(prayer_width) as u16;
        rosarium
            .style(Style::default().remove_modifier(Modifier::ITALIC).remove_modifier(Modifier::BOLD))
            .alignment(Alignment::Left)
    } else {
        rosarium
            .style(Style::default().add_modifier(Modifier::ITALIC).remove_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
    })
}


pub fn render_progress<'a>(rosary: &Rosary, window: &mut Window) -> Paragraph<'a> {
    let mut progress = Paragraph::new(
        if window.has_error() {
            window.error()
        } else {
            rosary.progress()
        }
    )
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Oratio")
                .border_type(BorderType::Rounded)
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
                .border_type(BorderType::Rounded)
        );
    progress
}

pub fn draw_rosary(terminal: &mut Terminal<CrosstermBackend<Stdout>>, rosary: &Rosary, window: &mut Window) -> Result<(), Box<dyn Error>> {

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Min(2),
                Constraint::Length(3),
            ]
                .as_ref(),
        );
    terminal.draw(|rect| {
        // Window layout
        let size = rect.size();
        let chunks = chunks.split(size);

        let bottom_bar = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),)
            .split(chunks[1]);

        let prayer_window = render_prayer(&rosary, window);
        if prayer_window.is_err() {
            window.set_error(prayer_window.as_ref().err().as_ref().unwrap().to_string());
        }
        rect.render_widget(prayer_window.unwrap(), chunks[0]);
        rect.render_widget(render_progress(&rosary, window), bottom_bar[0]);
        rect.render_widget(render_mysteries(), bottom_bar[1]);

        window.set_parent_dims(chunks[0].width, chunks[0].height);
    })?;
    Ok(())
}

pub fn draw_evening_prayer(terminal: &mut Terminal<CrosstermBackend<Stdout>>, window: &mut Window) -> Result<(), Box<dyn Error>> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Length(0),
            ]
                .as_ref(),
        );
    terminal.draw(|rect| {
        let size = rect.size();
        let chunks = chunks.split(size);
        let prayer_window = render_evening_prayer(window);
        if prayer_window.is_err() {
            window.set_error(prayer_window.as_ref().err().as_ref().unwrap().to_string());
        }
        rect.render_widget(prayer_window.unwrap(), chunks[0]);

        window.set_parent_dims(chunks[0].width, chunks[0].height);
    })?;
    Ok(())
}

pub fn refresh(terminal: &mut Terminal<CrosstermBackend<Stdout>>, rosary: &Rosary, window: &mut Window) -> Result<(), Box<dyn Error>> {
    terminal.clear()?;
    redraw(terminal, rosary, window)
}

pub fn redraw(terminal: &mut Terminal<CrosstermBackend<Stdout>>, rosary: &Rosary, window: &mut Window)
-> Result<(), Box<dyn Error>> {
        match window.active_menu_item() {
            MenuItem::Rosary => draw_rosary(terminal, rosary, window),
            MenuItem::Settings => {
                /*let layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                    )
                    .split(chunks[0]);*/
                Ok(())
            }
            MenuItem::Quit => {Ok(())},
            MenuItem::EveningPrayer => {draw_evening_prayer(terminal, window)}
        };
    Ok(())
}
