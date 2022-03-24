#![feature(iter_advance_by)]
extern crate core;

pub mod tui;
pub mod rosary;
pub mod language;
pub mod config;

#[cfg(test)]
mod tests {
    use crate::rosary::Rosary;

    #[test]
    fn rosary_forwards() {
        let mut rosary = Rosary::new();
        let mut rosary_forward: Vec<Rosary> = vec![rosary.clone()];
        let mut rosary_backward: Vec<Rosary> = vec![];
        loop {
            rosary.advance();
            rosary_forward.push(rosary.clone());

            if rosary.get_decade() == 5 && rosary.get_bead() == 12 {
                break;
            }
        }
        rosary_backward.push(rosary.clone());
        loop {
            rosary.recede();
            rosary_backward.push(rosary.clone());

            if rosary.get_decade() == 0 && rosary.get_bead() == 0 {
                break;
            }
        }
        rosary_backward.reverse();
        assert_eq!(rosary_forward, rosary_backward);
    }
}