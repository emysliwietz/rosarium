use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono::{Datelike, Weekday};
use tui::style::Color;
use crate::config::{MYSTERY_DIR, PRAYER_DIR, TITLE_FILE};
use crate::language::{ordinal_n_acc, ordinal_n_acc_upper, ordinal_n_gen};
use crate::rosary::Mysteries::{Glorious, Joyful, Luminous, Sorrowful};
use crate::rosary::Prayer::{ApostlesCreed, FatimaOMyJesus, FifthMystery, FinalPrayer, FirstMystery, FourthMystery, GloryBe, HailHolyQueen, HailMary, HailMaryCharity, HailMaryFaith, HailMaryHope, Laudetur, OurFather, SecondMystery, SignOfCross, ThirdMystery};


#[derive(Debug)]
pub enum Mysteries {
    Joyful,
    Sorrowful,
    Glorious,
    Luminous
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Prayer {
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
    FirstMystery,
    SecondMystery,
    ThirdMystery,
    FourthMystery,
    FifthMystery,
    Laudetur,
    FinalPrayer
}

impl Prayer {
    /// Return corresponding file name
    fn get_file(&self) -> String {
        match self {
            SignOfCross => String::from("signum_crucis"),
            ApostlesCreed => String::from("symbolum_apostolorum"),
            OurFather => String::from("pater_noster"),
            HailMary
                | HailMaryFaith
                | HailMaryHope
                | HailMaryCharity => String::from("ave_maria"),
            GloryBe => String::from("gloria_patri"),
            FatimaOMyJesus => String::from("oratio_fatimae"),
            HailHolyQueen => String::from("salve_regina"),
            FinalPrayer => String::from("oratio_ad_finem_rosarii"),
            Laudetur => String::from("laudetur_jesus_christus"),
            FirstMystery => get_daily_mystery_file("I"),
            SecondMystery => get_daily_mystery_file("II"),
            ThirdMystery => get_daily_mystery_file("III"),
            FourthMystery => get_daily_mystery_file("IV"),
            FifthMystery => get_daily_mystery_file("V"),
            _ => String::from("")
        }
    }

    pub fn is_mystery(&self) -> bool {
        self == &FirstMystery || self == &SecondMystery || self == &ThirdMystery || self == &FourthMystery || self == &FifthMystery
    }

    pub fn to_color(&self) -> Color {
        match get_daily_mystery_enum() {
            Luminous => Color::White,
            Glorious => Color::LightMagenta,
            Sorrowful => Color::Red,
            Joyful => Color::Magenta,
            _ => Color::White
        }
    }

    pub fn get_prayer_text(&self) -> String {
        let file = PRAYER_DIR.to_owned() + "/" + &self.get_file();
        fs::read_to_string(&file)
            .unwrap_or(format!("Unable find prayer {:?}\n at {}", self, &file))
    }

    pub fn get_prayer_title(&self) -> String {
        let filename = PRAYER_DIR.to_owned() + "/" + TITLE_FILE;
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(filename).expect("Unable to open title file");
        let reader = BufReader::new(file);

        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (index, line) in reader.lines().enumerate() {
            let line = line.expect("Error fetching line"); // Ignore errors.
            if line.starts_with(&(String::from(self.get_file()) + ":")) {
                let title = String::from(line.split(":").nth(1).unwrap_or("no title found"));
                return match self {
                    FirstMystery => format!("{} Mysterium nuntiatur:\n{}", ordinal_n_acc_upper(1), title),
                    SecondMystery => format!("{} Mysterium nuntiatur:\n{}", ordinal_n_acc_upper(2), title),
                    ThirdMystery => format!("{} Mysterium nuntiatur:\n{}", ordinal_n_acc_upper(3), title),
                    FourthMystery => format!("{} Mysterium nuntiatur:\n{}", ordinal_n_acc_upper(4), title),
                    FifthMystery => format!("{} Mysterium nuntiatur:\n{}", ordinal_n_acc_upper(5), title),
                    HailMaryFaith => format!("{} {}", title, "pro fide"),
                    HailMaryHope => format!("{} {}", title, "pro spe"),
                    HailMaryCharity => format!("{} {}", title, "pro caritate"),
                    _ => title
                }
            }
        }
        format!("No title found for prayer {}", self.get_file())
    }
}

impl ToString for Mysteries {
    fn to_string(&self) -> String {
        let mystery_adj = match self {
            Joyful => "Gaudiosa",
            Sorrowful => "Dolorosa",
            Glorious => "Gloriosa",
            Luminous => "Luminosa"
        };

        format!("Mysteria {}", mystery_adj)
    }
}

fn get_daily_mystery_enum() -> Mysteries {
    let current_time = chrono::offset::Local::now();
    let weekday = current_time.date().weekday();
    match weekday {
        Weekday::Mon => Joyful,
        Weekday::Tue => Sorrowful,
        Weekday::Wed => Glorious,
        Weekday::Thu => Luminous,
        Weekday::Fri => Sorrowful,
        Weekday::Sat => Joyful,
        Weekday::Sun => Glorious
    }
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
    String::from(MYSTERY_DIR) + "/" + &get_daily_mystery().to_lowercase().replace(" ", "_") + "_" + latin_numeral
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
        Rosary { decade: 0, bead: 0, prayer: 1, num_prayer: 2}
    }

    pub fn to_prayer(&self) -> Prayer {
        self.prayers_for_bead()[(self.prayer - 1) as usize].clone()
    }

    fn prayers_for_bead(&self) -> Vec<Prayer> {
        match self.decade {
            0 => {
                match self.bead {
                    0 => vec![SignOfCross, ApostlesCreed],
                    1 => vec![OurFather],
                    2 => vec![HailMaryFaith],
                    3 => vec![HailMaryHope],
                    4 => vec![HailMaryCharity],
                    5 => vec![GloryBe],
                    6 => vec![FirstMystery, OurFather],
                    _ => {vec![]}
                }
            },
            i if i <= 5 => {
                match self.bead {
                    0 => vec![match self.decade {
                        2 => SecondMystery,
                        3 => ThirdMystery,
                        4 => FourthMystery,
                        5 => FifthMystery,
                        _ => Prayer::None
                    }, OurFather],
                    i if i>=1 && i<=10 => vec![HailMary],
                    11 => vec![GloryBe, FatimaOMyJesus],
                    12 => if self.decade == 5 {
                        vec![HailHolyQueen, FinalPrayer, Laudetur, SignOfCross]
                    } else {vec![]}
                    _ => {vec![]}
                }
            },
            _ => {vec![]}
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
            0 => {
                match self.bead {
                    i if i <= 5 => self.bead += 1,
                    6 => {self.decade = 1; self.bead = 1;},
                    _ => {}
                }
            }
            i if i <= 5 => {
                match self.bead {
                    0 => self.bead+=1,
                    j if j < 11 => self.bead += 1,
                    11 => {
                        if self.decade < 5 {
                            self.decade += 1; if self.decade == 1 {
                                self.bead = 1;
                            } else {
                                self.bead = 0;
                            }
                        } else {
                            self.bead += 1;
                        }
                    },
                    _ => {}
                }
            }
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
            0 => {
                match self.bead {
                    0 => {}
                    i if i > 0 && i <= 6 => self.bead -= 1,
                    _ => {}
                }
            }
            i if i <= 5 => {
                match self.bead {
                    0 => {self.decade -= 1; if self.decade > 0 {
                        self.bead = 11;
                    } else {
                        self.bead = 6;
                    }},
                    j if j <= 12 => {
                        if self.decade == 1 && self.bead == 1 {
                            self.decade -= 1;
                            self.bead = 6;
                        } else {
                            self.bead -= 1
                        }
                    },
                    _ => {}
                }
            }
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
            location = format!("ad {} nodum {} decennii", ordinal_n_acc(self.bead), ordinal_n_gen(self.decade))
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