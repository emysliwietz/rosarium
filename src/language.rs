pub fn ordinal(i: u8) -> &'static str {
    match i {
        0 => "zeroth",
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "forth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => ""
    }
}