use network_type::get_network_type;
use osm_parse_config::OSMParseConfig;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml::{Yaml, YamlLoader};

pub fn create_config_from_file(filename: String) -> OSMParseConfig {
    let mut file = File::open(filename).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    create_config_from_string(file_content)
}

pub fn create_config_from_string(config: String) -> OSMParseConfig {
    let docs = YamlLoader::load_from_str(&config).unwrap();
    let doc = &docs[0];

    let allowed_highways = parse_allowed_highways(doc);
    let max_speed_map = parse_max_speeds(doc);
    let default_walking_speed = parse_default_walking_speed(doc);

    OSMParseConfig::new(allowed_highways, max_speed_map, default_walking_speed)
}

fn parse_allowed_highways(doc: &Yaml) -> HashMap<::NetworkType, HashSet<String>> {
    let mut allowed_highways: HashMap<::NetworkType, HashSet<String>> = HashMap::new();

    let allowed_highways_file = doc["allowed_highways"].as_vec().unwrap();
    for entry in allowed_highways_file {
        for (key, types) in entry.as_hash().unwrap() {
            let types = types
                .as_str()
                .unwrap()
                .split(", ")
                .map(|x| x.to_string())
                .collect::<HashSet<String>>();

            let network_type = get_network_type(key.as_str().unwrap());
            allowed_highways.insert(network_type, types);
        }
    }

    allowed_highways
}

fn parse_max_speeds(doc: &Yaml) -> HashMap<String, u8> {
    let mut max_speed = HashMap::new();
    let max_speeds_file = doc["max_speed"].as_vec().unwrap();
    for entry in max_speeds_file {
        for (key, speed) in entry.as_hash().unwrap() {
            let speed = speed.as_i64().unwrap();
            let t = key.as_str().unwrap();

            max_speed.insert(t.to_string(), speed as u8);
        }
    }
    max_speed
}

fn parse_default_walking_speed(doc: &Yaml) -> u8 {
    doc["default_walking_speed"].as_i64().unwrap() as u8
}
