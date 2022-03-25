use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::config::{PRAYER_DIR, TITLE_FILE};
use crate::tui::Window;

#[derive(Debug)]
pub enum Language {
    ANGLIA,
    GERMANA,
    LATINA,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::ANGLIA => "anglia",
            Language::GERMANA => "germana",
            Language::LATINA => "latina",
        }.to_owned()
    }
}

/// Ordinals for Latin neuter accusative singular
pub fn ordinal_n_acc(i: u8) -> &'static str {
    match i {
        1 => "primum",
        2 => "secundum",
        3 => "tertium",
        4 => "quartum",
        5 => "quintum",
        6 => "sextum",
        7 => "septimum",
        8 => "octavum",
        9 => "nonum",
        10 => "decimum",
        _ => ""
    }
}

pub fn ordinal_n_acc_upper(i: u8) -> String {
    let ordinal = String::from(ordinal_n_acc(i));
    ordinal[..1].to_uppercase() + &ordinal[1..]
}

/// Ordinals for Latin neuter genitive singular
pub fn ordinal_n_gen(i: u8) -> &'static str {
    match i {
        1 => "primi",
        2 => "secundi",
        3 => "tertii",
        4 => "quarti",
        5 => "quinti",
        6 => "sexti",
        7 => "septimi",
        8 => "octavi",
        9 => "noni",
        10 => "decimi",
        _ => ""
    }
}

pub fn get_title_translation(lookup: &str, window: &mut Window) -> String {
    let filename = PRAYER_DIR.to_owned() + "/" + &window.language() + "/" + TITLE_FILE;
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(&filename);
    if file.is_err() {
        window.set_error(format!("Unable to open title file: {}", &filename));
        return String::from("");
    }
    let file = file.unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.expect("Error fetching line"); // Ignore errors.
        if line.starts_with(&(String::from(lookup) + ":")) {
            return String::from(line.split(":").nth(1).unwrap_or("no title found").trim());
        }
    }
    return format!("No title found for prayer {}", lookup)
}