use crate::rosary::Mysteries;
use crate::rosary::Mysteries::{Glorious, Joyful, Luminous, Sorrowful};
use crate::tui::{e, E};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Weekday};

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
    let christmas = NaiveDate::from_ymd_opt(time.year(), 12, 24)?;
    let mut fourth_advent = christmas.clone();
    while fourth_advent.weekday() != Weekday::Sun {
        fourth_advent = fourth_advent.pred_opt()?;
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

pub struct AnnusLiturgicus {
    festum_circumcisionis_domini: NaiveDate,
    epiphan_domini: NaiveDate,
    // Purificatio Mariae
    praesentatio_domini: NaiveDate,
    septuagesima: NaiveDate,
    sexagesima: NaiveDate,
    quinquagesima: NaiveDate,
    dies_cinerum: NaiveDate,
    quadragesima: NaiveDate,
    dominica_reminiscere: NaiveDate,
    dominica_oculi: NaiveDate,
    dominica_laetare: NaiveDate,
    annuntiatio_beatae_mariae_virginis: NaiveDate,
    dominica_de_passione: NaiveDate,
    dominica_in_palmis_de_passione_domini: NaiveDate,
    dies_cenae_domini: NaiveDate,
    dies_passionis_domini: NaiveDate,
    dominica_resurrectionis_domini: NaiveDate,
    easter_monday: NaiveDate,
    easter_tuesday: NaiveDate,
    dominica_in_albis: NaiveDate,
    dominica_misericordia: NaiveDate,
    dominica_jubilate: NaiveDate,
    dominica_cantate: NaiveDate,
    dominica_rogate: NaiveDate,
    ascensio_domini: NaiveDate,
    dominica_exaudi: NaiveDate,
    pentecostes: NaiveDate,
    dominica_trinitatis: NaiveDate,
    nativitas_ioannis_baptistae: NaiveDate,
    festum_michaeli: NaiveDate,
    omnium_sanctorum: NaiveDate,
    festum_sancti_martini: NaiveDate,
    first_advent: NaiveDate,
    second_advent: NaiveDate,
    third_advent: NaiveDate,
    fourth_advent: NaiveDate,
    festum_nativitatis_domini: NaiveDate,
    festum_st_johannis_evangelistae: NaiveDate,
    third_christmas_day: NaiveDate,
}

type LiturgicalDate = (&'static str, NaiveDate);

impl AnnusLiturgicus {
    pub fn new(year: i32) -> Result<AnnusLiturgicus, E> {
        let easter = pascha(year)?;
        let dies_cinerum = days_before(easter, 46)?;
        let quinquagesima = sunday_before(dies_cinerum)?;
        let pentecostes = days_after(easter, 49)?;
        let festum_nativitatis_domini =
            NaiveDate::from_ymd_opt(year, 12, 25).ok_or("no nativity date")?;
        let fourth_advent = sunday_before(festum_nativitatis_domini)?;

        Ok(AnnusLiturgicus {
            festum_circumcisionis_domini: NaiveDate::from_ymd_opt(year, 1, 1)
                .ok_or("no circumcision date")?,
            epiphan_domini: NaiveDate::from_ymd_opt(year, 1, 6).ok_or("no epiphany date")?,
            // Purificatio Mariae
            praesentatio_domini: NaiveDate::from_ymd_opt(year, 2, 2)
                .ok_or("no presentatio date")?,
            septuagesima: weeks_before(quinquagesima, 2)?,
            sexagesima: weeks_before(quinquagesima, 1)?,
            quinquagesima,
            dies_cinerum,
            quadragesima: weeks_before(easter, 6)?,
            dominica_reminiscere: weeks_before(easter, 5)?,
            dominica_oculi: weeks_before(easter, 4)?,
            dominica_laetare: weeks_before(easter, 3)?,
            annuntiatio_beatae_mariae_virginis: NaiveDate::from_ymd_opt(year, 3, 25)
                .ok_or("no annuntiatio beatae mariae date")?,
            dominica_de_passione: weeks_before(easter, 2)?,
            dominica_in_palmis_de_passione_domini: sunday_before(easter)?,
            dies_cenae_domini: weekday_before(easter, Weekday::Thu)?,
            dies_passionis_domini: weekday_before(easter, Weekday::Fri)?,
            dominica_resurrectionis_domini: easter,
            easter_monday: weekday_after(easter, Weekday::Mon)?,
            easter_tuesday: weekday_after(easter, Weekday::Tue)?,
            dominica_in_albis: sunday_after(easter)?,
            dominica_misericordia: weeks_after(easter, 2)?,
            dominica_jubilate: weeks_after(easter, 3)?,
            dominica_cantate: weeks_after(easter, 4)?,
            dominica_rogate: weeks_after(easter, 5)?,
            ascensio_domini: days_after(easter, 39)?,
            dominica_exaudi: weeks_after(easter, 6)?,
            pentecostes,
            dominica_trinitatis: sunday_after(pentecostes)?,
            nativitas_ioannis_baptistae: NaiveDate::from_ymd_opt(year, 6, 24)
                .ok_or("no nativity date of John the Baptist")?,
            festum_michaeli: NaiveDate::from_ymd_opt(year, 9, 29)
                .ok_or("No date for feast of St. Michael")?,
            omnium_sanctorum: NaiveDate::from_ymd_opt(year, 11, 1)
                .ok_or("No date for all saints")?,
            festum_sancti_martini: NaiveDate::from_ymd_opt(year, 11, 11)
                .ok_or("No date for St. Martin")?,
            first_advent: weeks_before(fourth_advent, 3)?,
            second_advent: weeks_before(fourth_advent, 2)?,
            third_advent: weeks_before(fourth_advent, 1)?,
            fourth_advent,
            festum_nativitatis_domini,
            festum_st_johannis_evangelistae: days_after(festum_nativitatis_domini, 1)?,
            third_christmas_day: days_after(festum_nativitatis_domini, 2)?,
        })
    }

    pub fn to_vec(&self) -> Vec<LiturgicalDate> {
        vec![
            (
                "festum_circumcisionis_domini",
                self.festum_circumcisionis_domini,
            ),
            ("epiphan_domini", self.epiphan_domini),
            ("praesentatio_domini", self.praesentatio_domini),
            ("septuagesima", self.septuagesima),
            ("sexagesima", self.sexagesima),
            ("quinquagesima", self.quinquagesima),
            ("dies_cinerum", self.dies_cinerum),
            ("quadragesima", self.quadragesima),
            ("dominica_reminiscere", self.dominica_reminiscere),
            ("dominica_oculi", self.dominica_oculi),
            ("dominica_laetare", self.dominica_laetare),
            (
                "annuntiatio_beatae_mariae_virginis",
                self.annuntiatio_beatae_mariae_virginis,
            ),
            ("dominica_de_passione", self.dominica_de_passione),
            (
                "dominica_in_palmis_de_passione_domini",
                self.dominica_in_palmis_de_passione_domini,
            ),
            ("dies_cenae_domini", self.dies_cenae_domini),
            ("dies_passionis_domini", self.dies_passionis_domini),
            (
                "dominica_resurrectionis_domini",
                self.dominica_resurrectionis_domini,
            ),
            ("easter_monday", self.easter_monday),
            ("easter_tuesday", self.easter_tuesday),
            ("dominica_in_albis", self.dominica_in_albis),
            ("dominica_misericordia", self.dominica_misericordia),
            ("dominica_jubilate", self.dominica_jubilate),
            ("dominica_cantate", self.dominica_cantate),
            ("dominica_rogate", self.dominica_rogate),
            ("ascensio_domini", self.ascensio_domini),
            ("dominica_exaudi", self.dominica_exaudi),
            ("pentecostes", self.pentecostes),
            ("dominica_trinitatis", self.dominica_trinitatis),
            (
                "nativitas_ioannis_baptistae",
                self.nativitas_ioannis_baptistae,
            ),
            ("festum_michaeli", self.festum_michaeli),
            ("omnium_sanctorum", self.omnium_sanctorum),
            ("festum_sancti_martini", self.festum_sancti_martini),
            ("first_advent", self.first_advent),
            ("second_advent", self.second_advent),
            ("third_advent", self.third_advent),
            ("fourth_advent", self.fourth_advent),
            ("festum_nativitatis_domini", self.festum_nativitatis_domini),
            (
                "festum_st_johannis_evangelistae",
                self.festum_st_johannis_evangelistae,
            ),
            ("third_christmas_day", self.third_christmas_day),
        ]
    }
}

fn weekday_before(date: NaiveDate, weekday: Weekday) -> Result<NaiveDate, E> {
    let mut new_date = date
        .clone()
        .pred_opt()
        .ok_or_else(|| e("Can't calculate date"))?;
    while new_date.weekday() != weekday {
        new_date = new_date
            .pred_opt()
            .ok_or_else(|| e("Can't calculate date"))?;
    }
    Ok(new_date)
}

fn weekday_after(date: NaiveDate, weekday: Weekday) -> Result<NaiveDate, E> {
    let mut new_date = date
        .clone()
        .succ_opt()
        .ok_or_else(|| e("Can't calculate date"))?;
    while new_date.weekday() != weekday {
        new_date = new_date
            .succ_opt()
            .ok_or_else(|| e("Can't calculate date"))?;
    }
    Ok(new_date)
}

fn sunday_before(date: NaiveDate) -> Result<NaiveDate, E> {
    weekday_before(date, Weekday::Sun)
}

fn sunday_after(date: NaiveDate) -> Result<NaiveDate, E> {
    weekday_after(date, Weekday::Sun)
}

fn weeks_before(date: NaiveDate, weeks: i64) -> Result<NaiveDate, E> {
    let d = date.clone();
    d.checked_sub_signed(Duration::weeks(weeks))
        .ok_or_else(|| e("Can't calculate date"))
}

fn weeks_after(date: NaiveDate, weeks: i64) -> Result<NaiveDate, E> {
    let d = date.clone();
    d.checked_add_signed(Duration::weeks(weeks))
        .ok_or_else(|| e("Can't calculate date"))
}

fn days_before(date: NaiveDate, days: i64) -> Result<NaiveDate, E> {
    let d = date.clone();
    d.checked_sub_signed(Duration::days(days))
        .ok_or_else(|| e("Can't calculate date"))
}

fn days_after(date: NaiveDate, days: i64) -> Result<NaiveDate, E> {
    let d = date.clone();
    d.checked_add_signed(Duration::days(days))
        .ok_or_else(|| e("Can't calculate date"))
}

fn pascha(year: i32) -> Result<NaiveDate, E> {
    bdays::easter::easter_naive_date(year).map_err(|_| e("Can't calculate Easter"))
}
