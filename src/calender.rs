use crate::rosary::Mysteries;
use crate::rosary::Mysteries::{Glorious, Joyful, Luminous, Sorrowful};
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
    pub fn new(year: DateTime<Local>) -> AnnusLiturgicus {
        let easter = pascha(year).expect("Wrong date");
        let dies_cinerum = days_before(easter, 46);
        let quinquagesima = sunday_before(dies_cinerum);
        let pentecostes = days_after(easter, 49);
        let festum_nativitatis_domini = NaiveDate::from_ymd(year.year(), 12, 25);
        let fourth_advent = sunday_before(festum_nativitatis_domini);

        AnnusLiturgicus {
            festum_circumcisionis_domini: NaiveDate::from_ymd(year.year(), 1, 1),
            epiphan_domini: NaiveDate::from_ymd(year.year(), 1, 6),
            // Purificatio Mariae
            praesentatio_domini: NaiveDate::from_ymd(year.year(), 1, 6),
            septuagesima: weeks_before(quinquagesima, 2),
            sexagesima: weeks_before(quinquagesima, 1),
            quinquagesima,
            dies_cinerum,
            quadragesima: weeks_before(easter, 6),
            dominica_reminiscere: weeks_before(easter, 5),
            dominica_oculi: weeks_before(easter, 4),
            dominica_laetare: weeks_before(easter, 3),
            annuntiatio_beatae_mariae_virginis: NaiveDate::from_ymd(year.year(), 3, 25),
            dominica_de_passione: weeks_before(easter, 2),
            dominica_in_palmis_de_passione_domini: sunday_before(easter),
            dies_cenae_domini: weekday_before(easter, Weekday::Thu),
            dies_passionis_domini: weekday_before(easter, Weekday::Fri),
            dominica_resurrectionis_domini: easter,
            easter_monday: weekday_after(easter, Weekday::Mon),
            easter_tuesday: weekday_after(easter, Weekday::Tue),
            dominica_in_albis: sunday_after(easter),
            dominica_misericordia: weeks_after(easter, 2),
            dominica_jubilate: weeks_after(easter, 3),
            dominica_cantate: weeks_after(easter, 4),
            dominica_rogate: weeks_after(easter, 5),
            ascensio_domini: days_after(easter, 39),
            dominica_exaudi: weeks_after(easter, 6),
            pentecostes,
            dominica_trinitatis: sunday_after(pentecostes),
            nativitas_ioannis_baptistae: NaiveDate::from_ymd(year.year(), 6, 24),
            festum_michaeli: NaiveDate::from_ymd(year.year(), 9, 29),
            omnium_sanctorum: NaiveDate::from_ymd(year.year(), 11, 1),
            festum_sancti_martini: NaiveDate::from_ymd(year.year(), 11, 11),
            first_advent: weeks_before(fourth_advent, 3),
            second_advent: weeks_before(fourth_advent, 2),
            third_advent: weeks_before(fourth_advent, 1),
            fourth_advent,
            festum_nativitatis_domini,
            festum_st_johannis_evangelistae: days_after(festum_nativitatis_domini, 1),
            third_christmas_day: days_after(festum_nativitatis_domini, 2),
        }
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

fn weekday_before(date: NaiveDate, weekday: Weekday) -> NaiveDate {
    let mut new_date = date.clone().pred();
    while new_date.weekday() != weekday {
        new_date = new_date.pred();
    }
    return new_date;
}

fn weekday_after(date: NaiveDate, weekday: Weekday) -> NaiveDate {
    let mut new_date = date.clone().succ();
    while new_date.weekday() != weekday {
        new_date = new_date.succ();
    }
    return new_date;
}

fn sunday_before(date: NaiveDate) -> NaiveDate {
    weekday_before(date, Weekday::Sun)
}

fn sunday_after(date: NaiveDate) -> NaiveDate {
    weekday_after(date, Weekday::Sun)
}

fn weeks_before(date: NaiveDate, weeks: i64) -> NaiveDate {
    let d = date.clone();
    d.checked_sub_signed(Duration::weeks(weeks))
        .expect("Wrong date calculation")
}

fn weeks_after(date: NaiveDate, weeks: i64) -> NaiveDate {
    let d = date.clone();
    d.checked_add_signed(Duration::weeks(weeks))
        .expect("Wrong date calculation")
}

fn days_before(date: NaiveDate, days: i64) -> NaiveDate {
    let d = date.clone();
    d.checked_sub_signed(Duration::days(days))
        .expect("Wrong date calculation")
}

fn days_after(date: NaiveDate, days: i64) -> NaiveDate {
    let d = date.clone();
    d.checked_add_signed(Duration::days(days))
        .expect("Wrong date calculation")
}

fn pascha(year: DateTime<Local>) -> Option<NaiveDate> {
    let easter = bdays::easter::easter_naive_date(year.year());
    if easter.is_err() {
        return None;
    }
    return Some(easter.unwrap());
}
