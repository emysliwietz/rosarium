use std::fs::read_to_string;

use chrono::Datelike;
use linked_hash_map::LinkedHashMap;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use yaml_rust::{Yaml, YamlLoader};

use crate::prayer::{Prayer, _Prayer};

const E: &str = "Malformed YAML";
type PrayerList = Vec<Box<dyn Prayer>>;

pub fn load() {
    let s = read_to_string("preces/.config.yaml").unwrap();
    let docs: Vec<Yaml> = YamlLoader::load_from_str(&s).unwrap();
    println!("{:?}", get_order(&docs[0]));
}

/// Return a list of all prayer set titiles
pub fn get_all_prayset_titles(y: &Vec<Yaml>) -> Vec<String> {
    let mut titles: Vec<String> = vec![];
    for i in 0..y.len() {
        let title = y[i]["title"].as_str();
        if title.is_some() {
            titles.push(String::from(title.unwrap()));
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
pub fn get_order(y: &Yaml) -> PrayerList {
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
            order.append(&mut process_group(y, prayer));
        }
    }
    order
}

/// Process a prayer group definition in "order"
pub fn process_group(y: &Yaml, g: &LinkedHashMap<Yaml, Yaml>) -> PrayerList {
    let mut order: PrayerList = vec![];
    for (var_name, properties) in g.iter() {
        let var_name = var_name.as_str().expect(E);
        let group_prayers = expand_group_definition(y, var_name);
        order.append(&mut pick_and_apply_properties(&group_prayers, &properties));
    }
    order
}

pub fn expand_group_definition(y: &Yaml, g: &str) -> PrayerList {
    let mut order: PrayerList = vec![];
    for prayer in y["prayers"][g].as_vec().expect(E) {
        order.push(Box::new(_Prayer::new(
            prayer.as_str().expect(E).to_string(),
        )))
    }
    order
}

pub fn pick_and_apply_properties<'a>(group: &PrayerList, properties: &Yaml) -> PrayerList {
    let mut order: PrayerList = vec![];
    let count = properties["count"].as_i64().unwrap_or(group.len() as i64) as usize;
    let random = properties["random"].as_bool().unwrap_or(false);
    if random {
        let today = chrono::offset::Local::now()
            .date()
            .naive_local()
            .num_days_from_ce() as u64;
        let mut rng = StdRng::seed_from_u64(today);
        for p in group.choose_multiple(&mut rng, count) {
            order.push(p.clone());
        }
    } else {
        order.append(&mut group[0..count].to_vec());
    }
    order
}
