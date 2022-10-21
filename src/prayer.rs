use crate::rosary::RosaryPrayer;
use crate::{
    config::PRAYER_DIR,
    language::{get_title_translation, Language},
    tui::Window,
};
use chrono::Datelike;
use rand::{rngs::StdRng, seq::SliceRandom, thread_rng, SeedableRng};
use soloud::{audio, AudioExt, LoadExt, Wav, WavStream};
use std::fs;
use std::path::Path;
use std::str::FromStr;

pub trait Prayer {
    fn get_file(&self) -> String;
    fn get_prayer_title(&self, window: &Window) -> String {
        get_title_translation(&self.get_file(), window)
    }

    fn load_audio(&self, window: &mut Window) -> Option<WavStream> {
        let audio_file = PRAYER_DIR.to_owned()
            + "/"
            + &window.language()
            + "/cantus/"
            + &self.get_file()
            + ".wav";
        if Path::new(&audio_file).exists() {
            let mut wav = audio::WavStream::default();
            let audio = wav.load(&std::path::Path::new(&audio_file));
            if audio.is_ok() {
                Some(wav)
            } else {
                None
            }
        } else {
            self.load_fallback_prayer_audio(window)
        }
    }

    fn load_fallback_prayer_audio(&self, window: &mut Window) -> Option<WavStream> {
        for lan in Language::VALUES.iter() {
            let audio_file = PRAYER_DIR.to_owned()
                + "/"
                + &lan.to_string()
                + "/cantus/"
                + &self.get_file()
                + ".wav";
            if Path::new(&audio_file).exists() {
                let mut wav = audio::WavStream::default();
                let audio = wav.load(&std::path::Path::new(&audio_file));
                if audio.is_ok() {
                    window.set_language(lan);
                    return Some(wav);
                }
            }
        }
        None
    }

    fn get_prayer_text(&self, window: &mut Window) -> String {
        let file = PRAYER_DIR.to_owned() + "/" + &window.language() + "/" + &self.get_file();
        fs::read_to_string(&file).unwrap_or(self.get_fallback_prayer_text(window))
    }

    fn get_prayer_text_for_language(&self, lang: &Language) -> String {
        let file = PRAYER_DIR.to_owned() + "/" + &lang.to_string() + "/" + &self.get_file();
        fs::read_to_string(&file).unwrap_or(format!("{} not found", lang.to_string()))
    }

    fn title_text_audio(&self, window: &mut Window) -> (String, String, Option<WavStream>) {
        let audio = self.load_audio(window);
        let text = if audio.is_none() {
            self.get_prayer_text(window)
        } else {
            self.get_prayer_text_for_language(window.get_language())
        };
        let title = self.get_prayer_title(window);
        (title, text, audio)
    }

    fn get_fallback_prayer_text(&self, window: &mut Window) -> String {
        for lan in Language::VALUES.iter() {
            let file = PRAYER_DIR.to_owned() + "/" + &lan.to_string() + "/" + &self.get_file();
            let prayer_text = fs::read_to_string(&file);
            if prayer_text.is_ok() {
                window.set_language(lan);
                return prayer_text.unwrap();
            }
        }
        format!("Unable find prayer at {}", &self.get_file())
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
    fn new(file: String) -> Self {
        _Prayer { file }
    }
}

/// Convert reference to Prayer type object to owned Prayer via cloning
pub fn to_owned(b: &Box<dyn Prayer>) -> Box<dyn Prayer> {
    Box::new(_Prayer::new(b.get_file()))
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EveningPrayers {
    None,
    OratioIesu,
    PrayerBeforeSleep,
    TropariaBeforeSleep,
    StMacariusTheGreat,
    StAntiochus,
}

impl Prayer for EveningPrayers {
    fn get_file(&self) -> String {
        String::from(match self {
            EveningPrayers::OratioIesu => "oratio_Iesu",
            EveningPrayers::PrayerBeforeSleep => "jordanville/prayer_before_sleep",
            EveningPrayers::TropariaBeforeSleep => "jordanville/troparia_before_sleep",
            EveningPrayers::StMacariusTheGreat => "jordanville/st_macarius_the_great",
            EveningPrayers::StAntiochus => "jordanville/st_antiochus",
            _ => "",
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct EveningPrayer {
    /// Number of current prayer
    curr_prayer: u8,
    prayers: Vec<Box<dyn Prayer>>,
}

impl EveningPrayer {
    pub fn new() -> EveningPrayer {
        let final_prayers = vec![
            EveningPrayers::StMacariusTheGreat,
            EveningPrayers::StAntiochus,
            EveningPrayers::OratioIesu,
        ];

        let mut ep = EveningPrayer {
            curr_prayer: 0,
            prayers: vec![
                //Box::new(RosaryPrayer::SignOfCross),
                Box::new(EveningPrayers::PrayerBeforeSleep),
                Box::new(EveningPrayers::TropariaBeforeSleep),
            ],
        };
        let today = chrono::offset::Local::now()
            .date()
            .naive_local()
            .num_days_from_ce() as u64;
        let mut rng = StdRng::seed_from_u64(today);
        ep.prayers
            .push(Box::new(final_prayers.choose(&mut rng).unwrap().clone()));
        ep
    }

    pub fn to_prayer(&self) -> Box<dyn Prayer> {
        to_owned(self.prayers.get(self.curr_prayer as usize).unwrap())
    }

    pub fn advance(&mut self) {
        if (self.curr_prayer as usize) < self.prayers.len() - 1 {
            self.curr_prayer += 1
        }
    }

    pub fn recede(&mut self) {
        if self.curr_prayer > 0 {
            self.curr_prayer -= 1
        }
    }
}
