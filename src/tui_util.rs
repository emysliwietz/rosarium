use tui::{
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{language::get_title_translation, tui::Window};

pub fn paragraph<'a>(text: String, title: &str, w: &mut Window) -> Paragraph<'a> {
    Paragraph::new(Text::from(text))
        .wrap(Wrap { trim: false })
        .scroll(w.get_offset())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(get_title_translation(title, w.get_language()))
                .border_type(tui::widgets::BorderType::Rounded),
        )
}

pub fn cursive<'a>(p: Paragraph) -> Paragraph {
    p
}

pub fn title_from_s<'a>(t: String, w: &Window) -> Text<'a> {
    let mut t = Text::from(hcenter(&String::from("\n".to_owned() + &t), w));
    t.patch_style(
        Style::default()
            .remove_modifier(Modifier::ITALIC)
            .add_modifier(Modifier::BOLD)
            .fg(Color::LightYellow),
    );
    t
}

pub fn cursive_p<'a>(
    text: String,
    border_title: &str,
    title: String,
    w: &mut Window,
) -> Paragraph<'a> {
    let title = title_from_s(title, w);
    let mut text = Text::from(hcenter(&text, w));
    text.patch_style(
        Style::default()
            .add_modifier(Modifier::ITALIC)
            .remove_modifier(Modifier::BOLD)
            .fg(Color::White),
    );
    combine_to_p(text, title, border_title, w)
}

pub fn hcenter(text: &String, window: &Window) -> String {
    let mut text_width = 0;
    for line in text.lines() {
        if text_width < line.len() {
            text_width = line.len();
        }
    }
    let v_offset = window.get_vert_offset(text_width);
    let offset_string = " ".repeat(v_offset);
    offset_string.clone() + &text.replace("\n", &("\n".to_owned() + &offset_string))
}

// Combines title and text, vertical centering both, as paragraph
pub fn combine_to_p<'a>(
    text: Text<'a>,
    mut title: Text<'a>,
    border_title: &str,
    w: &mut Window,
) -> Paragraph<'a> {
    let p = Text::raw("\n".repeat(w.get_top_offset(text.height() + 4)));
    title.extend(p);
    title.extend(text);
    Paragraph::new(title)
        .wrap(Wrap { trim: false })
        .scroll(w.get_offset())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(get_title_translation(border_title, w.get_language()))
                .border_type(tui::widgets::BorderType::Rounded),
        )
}

pub fn center_p<'a>(text: &str, title: &str, w: &mut Window) -> Paragraph<'a> {
    paragraph(hcenter(&String::from(text), w), title, w)
}
