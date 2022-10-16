use std::error::Error;
use std::fs;

use crate::calender::get_daily_mystery_enum;
use tui::style::Color;

use crate::config::{INITIUM_FILE, MYSTERY_DIR, PRAYER_DIR};
use crate::language::{get_title_translation, ordinal_n_acc, ordinal_n_acc_upper, ordinal_n_gen};
use crate::rosary::Mysteries::{Glorious, Joyful, Luminous, Sorrowful};
use crate::rosary::RosaryPrayer::{
    ApostlesCreed, FatimaOMyJesus, FifthMystery, FinalPrayer, FirstMystery, FourthMystery, GloryBe,
    HailHolyQueen, HailMary, HailMaryCharity, HailMaryFaith, HailMaryHope, Laudetur, OurFather,
    PrayerForPriests, PrayerToStJoseph, PrayerToStMichael, SecondMystery, SignOfCross,
    ThirdMystery,
};
use crate::tui::Window;

#[derive(Debug)]
pub enum Mysteries {
    Joyful,
    Sorrowful,
    Glorious,
    Luminous,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RosaryPrayer {
    None,
    SignOfCross,
    ApostlesCreed,
    OurFather,
    HailMary,
    HailMaryFaith,
    HailMaryHope,
    HailMaryCharity,
    GloryBe,
    FatimaOMyJesus,
    HailHolyQueen,
    PrayerToStMichael,
    PrayerToStJoseph,
    PrayerForPriests,
    FirstMystery,
    SecondMystery,
    ThirdMystery,
    FourthMystery,
    FifthMystery,
    Laudetur,
    FinalPrayer,
}

impl RosaryPrayer {
    /// Return corresponding file name
    fn get_file(&self) -> String {
        match self {
            SignOfCross => String::from("signum_crucis"),
            ApostlesCreed => String::from("symbolum_apostolorum"),
            OurFather => String::from("pater_noster"),
            HailMary | HailMaryFaith | HailMaryHope | HailMaryCharity => String::from("ave_maria"),
            GloryBe => String::from("gloria_patri"),
            FatimaOMyJesus => String::from("oratio_fatimae"),
            HailHolyQueen => String::from("salve_regina"),
            PrayerToStMichael => String::from("oratio_ad_sanctum_michael"),
            PrayerForPriests => String::from("oratio_pro_sacerdotibus"),
            PrayerToStJoseph => String::from("oratio_ad_sanctum_iosephum"),
            FinalPrayer => String::from("oratio_ad_finem_rosarii"),
            Laudetur => String::from("laudetur_Iesus_Christus"),
            FirstMystery => get_daily_mystery_file("I"),
            SecondMystery => get_daily_mystery_file("II"),
            ThirdMystery => get_daily_mystery_file("III"),
            FourthMystery => get_daily_mystery_file("IV"),
            FifthMystery => get_daily_mystery_file("V"),
            _ => String::from(""),
        }
    }

    pub fn is_mystery(&self) -> bool {
        self == &FirstMystery
            || self == &SecondMystery
            || self == &ThirdMystery
            || self == &FourthMystery
            || self == &FifthMystery
    }

    pub fn to_color(&self) -> Color {
        match get_daily_mystery_enum() {
            Luminous => Color::White,
            Glorious => Color::LightMagenta,
            Sorrowful => Color::Red,
            Joyful => Color::Magenta,
        }
    }

    pub fn get_prayer_text(&self, window: &Window) -> Result<String, Box<dyn Error>> {
        let file = PRAYER_DIR.to_owned() + "/" + &window.language() + "/" + &self.get_file();
        let text = fs::read_to_string(&file)
            .unwrap_or(format!("Unable find prayer {:?}\n at {}", self, &file));
        if self == &HailMary {
            let mystery_addition = fs::read_to_string(
                PRAYER_DIR.to_owned() + "/" + &window.language() + "/" + &get_mysteries_file(),
            );
            if mystery_addition.is_ok() {
                let mystery_addition = mystery_addition.unwrap();
                let mut mystery_additions = mystery_addition.split("\n");
                (mystery_additions.advance_by((window.rosary.decade - 1) as usize))
                    .expect("Mystery addition file incomplete");
                return Ok(text.replace(
                    "Jesus.",
                    &format!("Jesus,\n{}.", mystery_additions.next().unwrap_or("")),
                ));
            }
        } else if self == &HailMaryFaith {
            return initial_hail_mary_addition(0, window, text);
        } else if self == &HailMaryHope {
            return initial_hail_mary_addition(1, window, text);
        } else if self == &HailMaryCharity {
            return initial_hail_mary_addition(2, window, text);
        }
        Ok(text)
    }

    pub fn get_prayer_title(&self, window: &mut Window) -> String {
        let title = get_title_translation(&self.get_file(), window);
        return match self {
            FirstMystery => format!(
                "{} Mysterium nuntiatur:\n{}",
                ordinal_n_acc_upper(1),
                title.trim()
            ),
            SecondMystery => format!(
                "{} Mysterium nuntiatur:\n{}",
                ordinal_n_acc_upper(2),
                title.trim()
            ),
            ThirdMystery => format!(
                "{} Mysterium nuntiatur:\n{}",
                ordinal_n_acc_upper(3),
                title.trim()
            ),
            FourthMystery => format!(
                "{} Mysterium nuntiatur:\n{}",
                ordinal_n_acc_upper(4),
                title.trim()
            ),
            FifthMystery => format!(
                "{} Mysterium nuntiatur:\n{}",
                ordinal_n_acc_upper(5),
                title.trim()
            ),
            HailMaryFaith => format!("{} {}", title, get_title_translation("pro_fide", window)),
            HailMaryHope => format!("{} {}", title, get_title_translation("pro_spe", window)),
            HailMaryCharity => format!(
                "{} {}",
                title,
                get_title_translation("pro_caritate", window)
            ),
            _ => title,
        };
    }
}

fn initial_hail_mary_addition(
    n: usize,
    window: &Window,
    text: String,
) -> Result<String, Box<dyn Error>> {
    let mystery_addition = fs::read_to_string(
        PRAYER_DIR.to_owned() + "/" + &window.language() + "/" + MYSTERY_DIR + "/" + INITIUM_FILE,
    );
    if mystery_addition.is_ok() {
        let mystery_addition = mystery_addition.unwrap();
        let mut mystery_additions = mystery_addition.split("\n");
        mystery_additions
            .advance_by(n)
            .expect("Mystery addition file incomplete");
        return Ok(text.replace(
            "Jesus.",
            &format!("Jesus,\n{}.", mystery_additions.next().unwrap_or("")),
        ));
    }
    return Ok(text);
}

impl ToString for Mysteries {
    fn to_string(&self) -> String {
        let mystery_adj = match self {
            Joyful => "Gaudiosa",
            Sorrowful => "Dolorosa",
            Glorious => "Gloriosa",
            Luminous => "Luminosa",
        };

        format!("Mysteria {}", mystery_adj)
    }
}

fn get_mysteries_file() -> String {
    String::from(MYSTERY_DIR)
        + "/"
        + match get_daily_mystery_enum() {
            Joyful => "gaudii",
            Sorrowful => "doloris",
            Glorious => "gloriae",
            Luminous => "lucis",
        }
        + "_mysteria"
}

pub fn get_daily_mystery() -> String {
    get_daily_mystery_enum().to_string()
    /*
        Sundays of Advent and Christmas  JOYFUL
    Sundays of Lent  SORROWFUL
    Other Sundays  GLORIOUS
         */
}

pub fn get_daily_mystery_file(latin_numeral: &str) -> String {
    String::from(MYSTERY_DIR)
        + "/"
        + &get_daily_mystery().to_lowercase().replace(" ", "_")
        + "_"
        + latin_numeral
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rosary {
    /// current decade
    decade: u8,
    /// current bead within decade
    bead: u8,
    /// current prayer in case of multiple prayers per bead
    /// starts at 1, not 0
    prayer: u8,
    /// number of prayers on the current bead
    num_prayer: u8,
}

impl Rosary {
    pub fn new() -> Rosary {
        Rosary {
            decade: 0,
            bead: 0,
            prayer: 1,
            num_prayer: 2,
        }
    }

    pub fn to_prayer(&self) -> RosaryPrayer {
        self.prayers_for_bead()[(self.prayer - 1) as usize].clone()
    }

    fn prayers_for_bead(&self) -> Vec<RosaryPrayer> {
        match self.decade {
            0 => match self.bead {
                0 => vec![SignOfCross, ApostlesCreed],
                1 => vec![OurFather],
                2 => vec![HailMaryFaith],
                3 => vec![HailMaryHope],
                4 => vec![HailMaryCharity],
                5 => vec![GloryBe],
                6 => vec![FirstMystery, OurFather],
                _ => {
                    vec![]
                }
            },
            i if i <= 5 => match self.bead {
                0 => vec![
                    match self.decade {
                        2 => SecondMystery,
                        3 => ThirdMystery,
                        4 => FourthMystery,
                        5 => FifthMystery,
                        _ => RosaryPrayer::None,
                    },
                    OurFather,
                ],
                i if i >= 1 && i <= 10 => vec![HailMary],
                11 => vec![GloryBe, FatimaOMyJesus],
                12 => {
                    if self.decade == 5 {
                        vec![
                            HailHolyQueen,
                            PrayerToStJoseph,
                            PrayerToStMichael,
                            FinalPrayer,
                            Laudetur,
                            SignOfCross,
                        ]
                    } else {
                        vec![]
                    }
                }
                _ => {
                    vec![]
                }
            },
            _ => {
                vec![]
            }
        }
    }

    pub fn advance(&mut self) {
        if self.prayer < self.num_prayer {
            self.prayer += 1;
            return;
        }

        let old_bead = self.bead;
        let old_decade = self.decade;

        match self.decade {
            0 => match self.bead {
                i if i <= 5 => self.bead += 1,
                6 => {
                    self.decade = 1;
                    self.bead = 1;
                }
                _ => {}
            },
            i if i <= 5 => match self.bead {
                0 => self.bead += 1,
                j if j < 11 => self.bead += 1,
                11 => {
                    if self.decade < 5 {
                        self.decade += 1;
                        if self.decade == 1 {
                            self.bead = 1;
                        } else {
                            self.bead = 0;
                        }
                    } else {
                        self.bead += 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        if self.decade != old_decade || self.bead != old_bead {
            self.prayer = 1;
            self.num_prayer = self.prayers_for_bead().len() as u8;
        }
    }

    pub fn recede(&mut self) {
        if self.prayer > 1 {
            self.prayer -= 1;
            return;
        }

        let old_bead = self.bead;
        let old_decade = self.decade;

        match self.decade {
            0 => match self.bead {
                0 => {}
                i if i > 0 && i <= 6 => self.bead -= 1,
                _ => {}
            },
            i if i <= 5 => match self.bead {
                0 => {
                    self.decade -= 1;
                    if self.decade > 0 {
                        self.bead = 11;
                    } else {
                        self.bead = 6;
                    }
                }
                j if j <= 12 => {
                    if self.decade == 1 && self.bead == 1 {
                        self.decade -= 1;
                        self.bead = 6;
                    } else {
                        self.bead -= 1
                    }
                }
                _ => {}
            },
            _ => {}
        }

        if self.decade != old_decade || self.bead != old_bead {
            self.num_prayer = self.prayers_for_bead().len() as u8;
            self.prayer = self.num_prayer;
        }
    }

    pub fn progress(&self) -> String {
        let location;
        if self.decade == 0 && self.bead == 0 {
            location = String::from("ad crucifixum");
        } else if self.decade == 0 && self.bead == 1 {
            location = format!("ad {} nodum", ordinal_n_acc(self.bead));
        } else if self.decade == 0 && self.bead == 5 {
            location = String::from("post tergeminum granum");
        } else if self.decade == 0 && self.bead == 6 {
            location = format!("ad {} nodum", ordinal_n_acc(self.bead - 1));
        } else if self.decade == 0 && self.bead > 1 && self.bead <= 4 {
            location = format!("ad {} granum tergemini grani", ordinal_n_acc(self.bead - 1));
        } else if self.bead == 0 {
            location = format!("ante {} decennium", ordinal_n_acc(self.decade));
        } else if self.bead == 11 {
            location = format!("post {} decennium", ordinal_n_acc(self.decade));
        } else if self.bead == 12 {
            location = String::from("ad finem rosarii");
        } else {
            location = format!(
                "ad {} nodum {} decennii",
                ordinal_n_acc(self.bead),
                ordinal_n_gen(self.decade)
            )
        }
        format!("Manus {}.", location)
    }

    pub fn get_decade(&self) -> u8 {
        self.decade
    }

    pub fn get_bead(&self) -> u8 {
        self.bead
    }

    pub fn get_curr_prayer(&self) -> String {
        format!("{}/{}", self.prayer, self.num_prayer)
    }
}
