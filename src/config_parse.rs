use std::fs::read_to_string;

use chrono::Datelike;
use linked_hash_map::LinkedHashMap;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use yaml_rust::{Yaml, YamlLoader};

use crate::prayer::{Prayer, _Prayer};

const E: &str = "Malformed YAML";
type PrayerList = Vec<Box<dyn Prayer>>;

pub fn test() {
    let s = read_to_string("preces/.config.yaml").unwrap();
    let y: Vec<Yaml> = YamlLoader::load_from_str(&s).unwrap();
    println!("{:?}", y[0]["order"])
}

/// Return a list of all prayer set titles and corresponding YAML
pub fn get_all_prayset_titles() -> Vec<(String, Yaml)> {
    let s = read_to_string("preces/.config.yaml").unwrap();
    let y: Vec<Yaml> = YamlLoader::load_from_str(&s).unwrap();
    let mut titles: Vec<(String, Yaml)> = vec![];
    for i in 0..y.len() {
        let title = y[i]["title"].as_str();
        if title.is_some() {
            titles.push((String::from(title.unwrap()), y[i].clone()));
        }
    }
    return titles;
}

/// Return yaml page corresponding to title
pub fn get_yaml_for_title<'a>(title: &str, y: &'a Vec<Yaml>) -> Option<&'a Yaml> {
    for page in y {
        if page["title"].as_str() == Some(title) {
            return Some(&page);
        }
    }
    None
}

/// Return a list of prayers as defined in "order"
pub fn get_order(rng: &mut StdRng, y: &Yaml) -> PrayerList {
    let mut order: PrayerList = vec![];
    let o = &y["order"].as_vec();
    if o.is_none() {
        return order;
    }
    let o = o.unwrap();
    for p in 0..o.len() {
        let prayer = o[p].as_str();
        if prayer.is_some() {
            order.push(Box::new(_Prayer::new(prayer.unwrap().to_string())))
        } else {
            let prayer = o[p].as_hash().expect(E);
            //println!("Group: {:?}", prayer);
            order.append(&mut process_group(rng, y, prayer));
        }
    }
    order
}

/// Process a prayer group definition in "order"
pub fn process_group(rng: &mut StdRng, y: &Yaml, g: &LinkedHashMap<Yaml, Yaml>) -> PrayerList {
    let mut order: PrayerList = vec![];
    for (var_name, properties) in g.iter() {
        let var_name = var_name.as_str().expect(E);
        let group_prayers = expand_group_definition(y, var_name);
        order.append(&mut pick_and_apply_properties(
            rng,
            &group_prayers,
            &properties,
        ));
    }
    order
}

pub fn expand_group_definition(y: &Yaml, g: &str) -> PrayerList {
    let mut order: PrayerList = vec![];
    let prayers = &y["prayers"][g];
    if !prayers.is_badvalue() {
        for prayer in prayers.as_vec().expect(E) {
            order.push(Box::new(_Prayer::new(
                prayer.as_str().expect(E).to_string(),
            )))
        }
    } else {
        order.push(Box::new(_Prayer::new(g.to_string())))
    }
    order
}

struct Properties {
    /// Select a random number between count_min and count_max of Prayers
    count_min: usize,
    count_max: usize,
    /// Whether to select prayers at random from group
    random: bool,
    /// Chance (in percent) to select any prayers from group at all
    chance: i64,
}

fn get_properties(group: &PrayerList, properties: &Yaml) -> Properties {
    let mut count_min = group.len();
    let mut count_max = count_min;
    let _count = properties["count"].as_i64();
    if _count.is_some() {
        count_min = _count.unwrap() as usize;
        count_max = _count.unwrap() as usize;
    } else {
        let _count = properties["count"].as_str();
        if _count.is_some() {
            let _count: Vec<&str> = _count.unwrap().split("-").collect();
            count_min = _count[0].parse().expect(E);
            count_max = _count[1].parse().expect(E);
        }
    }

    //.unwrap_or(group.len() as i64) as usize;
    let random = properties["random"].as_bool().unwrap_or(false);
    let chance = properties["chance"].as_i64().unwrap_or(100);

    Properties {
        count_min,
        count_max,
        random,
        chance,
    }
}

pub fn pick_and_apply_properties<'a>(
    rng: &mut StdRng,
    group: &PrayerList,
    properties: &Yaml,
) -> PrayerList {
    let mut order: PrayerList = vec![];
    let p = get_properties(group, properties);

    if !rng.gen_bool(p.chance as f64 / 100.0) {
        return order;
    }

    let count: usize = rng.gen_range(p.count_min, p.count_max + 1);

    for i in 0..count {
        order.push(if p.random {
            group.choose(rng).expect(E).clone()
        } else {
            println!("{:?}", group.len());
            group[i % group.len()].clone()
        })
    }
    order
}
