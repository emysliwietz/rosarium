use crate::config_parse::get_order;
use crate::tui::E;
use crate::{
    config::PRAYER_DIR,
    language::{get_title_translation, Language},
};
use rand::rngs::StdRng;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use yaml_rust::Yaml;

pub trait Prayer {
    fn get_file(&self) -> String;

    fn load_audio(&self, lan: &Language) -> Option<String> {
        let audio_file =
            PRAYER_DIR.to_owned() + "/" + &lan.to_string() + "/cantus/" + &self.get_file() + ".wav";
        if Path::new(&audio_file).exists() {
            Some(audio_file)
        } else {
            self.load_fallback_prayer_audio(lan)
        }
    }

    fn load_fallback_prayer_audio(&self, _lan: &Language) -> Option<String> {
        for lan in Language::VALUES.iter() {
            let audio_file = PRAYER_DIR.to_owned()
                + "/"
                + &lan.to_string()
                + "/cantus/"
                + &self.get_file()
                + ".wav";
            if Path::new(&audio_file).exists() {
                return Some(audio_file);
            }
        }
        None
    }

    fn get_available_languages(&self) -> Vec<Language> {
        let mut languages = vec![];
        for lan in Language::VALUES.iter() {
            let audio_file =
                PRAYER_DIR.to_owned() + "/" + &lan.to_string() + "/" + &self.get_file() + ".wav";
            if Path::new(&audio_file).exists() {
                languages.push(lan.to_owned());
            }
        }
        return languages;
    }

    fn get_prayer_title(&self, lan: &Language) -> String {
        get_title_translation(&self.get_file(), lan)
    }

    fn get_prayer_text_title(&self, lan: &Language) -> (String, String) {
        let file = PRAYER_DIR.to_owned() + "/" + &lan.to_string() + "/" + &self.get_file();
        let text = fs::read_to_string(&file);
        if text.is_ok() {
            (text.unwrap(), self.get_prayer_title(lan))
        } else {
            let (text, lan) = self.get_fallback_prayer_text();
            (text, self.get_prayer_title(lan))
        }
    }

    fn get_prayer_text_for_language(&self, lang: &Language) -> String {
        let file = PRAYER_DIR.to_owned() + "/" + &lang.to_string() + "/" + &self.get_file();
        fs::read_to_string(&file).unwrap_or(format!("{} not found", lang.to_string()))
    }

    fn get_fallback_prayer_text(&self) -> (String, &Language) {
        for lan in Language::VALUES.iter() {
            let file = PRAYER_DIR.to_owned() + "/" + &lan.to_string() + "/" + &self.get_file();
            let prayer_text = fs::read_to_string(&file);
            if prayer_text.is_ok() {
                return (prayer_text.unwrap(), &lan);
            }
        }
        return (
            format!("Unable find prayer at {}", &self.get_file()),
            &Language::ANGLIA,
        );
    }

    fn title_text_audio(&self, lan: &Language) -> (String, String, Option<String>) {
        let audio = self.load_audio(lan);
        let (text, title) = self.get_prayer_text_title(lan);
        (title, text, audio)
    }
}

impl std::fmt::Debug for dyn Prayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_file())
    }
}

impl std::cmp::PartialEq for dyn Prayer {
    fn eq(&self, other: &Self) -> bool {
        self.get_file() == other.get_file()
    }
}

impl ToString for Box<dyn Prayer> {
    fn to_string(&self) -> String {
        String::from(self.get_file())
    }
}

impl FromStr for Box<dyn Prayer> {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Box::new(_Prayer::new(String::from(s))))
    }
}

impl Default for Box<dyn Prayer> {
    fn default() -> Self {
        Box::new(_Prayer::new("no prayers specified in config".to_string()))
    }
}

impl Clone for Box<dyn Prayer> {
    fn clone(&self) -> Self {
        Box::new(_Prayer::new(self.to_string()))
    }
}

impl std::cmp::Eq for dyn Prayer {}

/// A struct only used to construct generic Prayer type objects
pub struct _Prayer {
    file: String,
}

impl Prayer for _Prayer {
    fn get_file(&self) -> String {
        return String::from(&self.file);
    }
}

impl _Prayer {
    pub fn new(file: String) -> Self {
        _Prayer { file }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct PrayerSet {
    title: String,
    /// Number of current prayer
    curr_prayer: u8,
    prayers: Vec<Box<dyn Prayer>>,
}

impl PrayerSet {
    pub fn new(title: String, y: Yaml, rng: &mut StdRng) -> Result<PrayerSet, E> {
        Ok(PrayerSet {
            title,
            curr_prayer: 0,
            prayers: get_order(rng, &y)?,
        })
    }

    pub fn get_title(&self, lan: &Language) -> String {
        get_title_translation(&self.title, lan)
    }

    pub fn to_prayer(&self) -> Box<dyn Prayer> {
        let o = self.prayers.get(self.curr_prayer as usize);
        if o.is_some() {
            o.unwrap().clone()
        } else {
            Box::default()
        }
    }

    pub fn advance(&mut self) {
        if self.prayers.len() > 0 && (self.curr_prayer as usize) < self.prayers.len() - 1 {
            self.curr_prayer += 1
        }
    }

    pub fn recede(&mut self) {
        if self.curr_prayer > 0 {
            self.curr_prayer -= 1
        }
    }
}
