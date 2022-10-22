use crate::rosary::Mysteries;
use crate::rosary::Mysteries::{Glorious, Joyful, Luminous, Sorrowful};
use chrono::{DateTime, Datelike, Local, NaiveDate, Weekday};

pub fn weekday() -> &'static str {
    match chrono::offset::Local::now().weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

pub fn get_daily_mystery_enum() -> Mysteries {
    let current_time = chrono::offset::Local::now();
    if let Some(special) = special(current_time) {
        return special;
    }
    let weekday = chrono::offset::Local::now().weekday();
    match weekday {
        Weekday::Mon => Joyful,
        Weekday::Tue => Sorrowful,
        Weekday::Wed => Glorious,
        Weekday::Thu => Luminous,
        Weekday::Fri => Sorrowful,
        Weekday::Sat => Joyful,
        Weekday::Sun => Glorious,
    }
}

fn special(time: DateTime<Local>) -> Option<Mysteries> {
    let easter = bdays::easter::easter_naive_date(time.year());
    if easter.is_err() {
        return None;
    }
    // On Easter Sunday, pray Glorious
    let easter = easter.unwrap();
    if easter.eq(&time.naive_local().date()) {
        return Some(Glorious);
    }
    let ash_wednesday = easter.checked_sub_signed(chrono::Duration::days(46))?;
    let days_since_ash_wednesday = (time.naive_local().date() - ash_wednesday).num_days();
    let days_since_easter = (time.naive_local().date() - easter).num_days();
    // double checking
    // On Lent Sundays, pray Sorrowful
    if 0 > days_since_ash_wednesday
        && days_since_ash_wednesday <= 46
        && days_since_easter < 0
        && time.weekday() == Weekday::Sun
    {
        return Some(Sorrowful);
    }
    // On Advent and Christmas Sundays, pray Joyful
    let christmas = NaiveDate::from_ymd(time.year(), 12, 24);
    let mut fourth_advent = christmas.clone();
    while fourth_advent.weekday() != Weekday::Sun {
        fourth_advent = fourth_advent.pred();
    }
    let first_advent = fourth_advent.checked_sub_signed(chrono::Duration::weeks(3))?;
    let days_since_first_advent = (time.naive_local().date() - first_advent).num_days();
    let days_since_fourth_advent = (time.naive_local().date() - fourth_advent).num_days();
    if 0 > days_since_first_advent
        && days_since_first_advent <= 7 * 4
        && days_since_fourth_advent <= 0
        && time.weekday() == Weekday::Sun
    {
        return Some(Joyful);
    }
    None
}
