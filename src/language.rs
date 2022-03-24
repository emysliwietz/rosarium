use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::config::{PRAYER_DIR, TITLE_FILE};

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