use std::fs;
use chrono::{Datelike, Weekday};
use crate::language::ordinal;
use crate::rosary::Mysteries::{Glorious, Joyful, Luminous, Sorrowful};
use crate::rosary::Prayer::{ApostlesCreed, FatimaOMyJesus, FifthMystery, FinalPrayer, FirstMystery, FourthMystery, GloryBe, HailHolyQueen, HailMary, HailMaryCharity, HailMaryFaith, HailMaryHope, OurFather, SecondMystery, SignOfCross, ThirdMystery};

pub const ROSARY_CROSS: &str = "🕇✝♱✟🕆✞";
pub const ROSARY_BEAD: &str = "•";
pub const PRAYER_DIR: &str = "./preces/latina";

pub enum Mysteries {
    Joyful,
    Sorrowful,
    Glorious,
    Luminous
}

#[derive(Debug, Copy, Clone)]
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
    FinalPrayer
}

impl Prayer {
    /// Return corresponding file name
    fn get_file(&self) -> &str {
        match self {
            SignOfCross => "signum_crucis",
            ApostlesCreed => "symbolum_apostolorum",
            OurFather => "pater_noster",
            HailMary
                | HailMaryFaith
                | HailMaryHope
                | HailMaryCharity => "ave_maria",
            GloryBe => "gloria_patri",
            FatimaOMyJesus => "oratio_fatimae",
            HailHolyQueen => "salve_regina",
            FinalPrayer => "oratio_ad_finem_rosarii",
            _ => ""
        }
    }

    pub fn get_prayer_text(&self) -> String {
        let file = PRAYER_DIR.to_owned() + "/" + self.get_file();
        fs::read_to_string(&file)
            .unwrap_or(format!("Unable find prayer {:?}\n at {}", self, &file))
    }
}

impl ToString for Mysteries {
    fn to_string(&self) -> String {
        let mystery_adj = match self {
            Joyful => "Joyful",
            Sorrowful => "Sorrowful",
            Glorious => "Glorious",
            Luminous => "Luminous"
        };

        format!("{} Mystery of the Rosary", mystery_adj)
    }
}

pub fn get_daily_mystery() -> String {
    let current_time = chrono::offset::Local::now();
    let weekday = current_time.date().weekday();
    let mystery = match weekday {
        Weekday::Mon => Joyful,
        Weekday::Tue => Sorrowful,
        Weekday::Wed => Glorious,
        Weekday::Thu => Luminous,
        Weekday::Fri => Sorrowful,
        Weekday::Sat => Joyful,
        Weekday::Sun => Glorious
    };
    mystery.to_string()
    /*
    Sundays of Advent and Christmas  JOYFUL
Sundays of Lent  SORROWFUL
Other Sundays  GLORIOUS
     */
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
                    }],
                    i if i>=1 && i<=10 => vec![HailMary],
                    11 => vec![GloryBe, FatimaOMyJesus],
                    12 => if self.decade == 5 {
                        vec![HailHolyQueen, FinalPrayer, SignOfCross]
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
            location = String::from("the crucifix");
        } else if self.decade == 0 && self.bead == 1 {
            location = format!("the {} bead", ordinal(self.bead));
        } else if self.decade == 0 && self.bead == 5 {
            location = String::from("after the triplet");
        } else if self.decade == 0 && self.bead == 6 {
            location = format!("the {} bead", ordinal(self.bead - 1));
        } else if self.decade == 0 && self.bead > 1 && self.bead <= 4 {
            location = format!("the {} bead of the triplet", ordinal(self.bead - 1));
        } else if self.bead == 0 {
            location = format!("before the {} decade", ordinal(self.decade));
        } else if self.bead == 11 {
            location = format!("after the {} decade", ordinal(self.decade));
        } else if self.bead == 12 {
            location = String::from("the closing prayer");
        } else {
            location = format!("the {} bead of the {} decade", ordinal(self.bead), ordinal(self.decade))
        }
        format!("Praying {}.", location)
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