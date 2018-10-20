use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml::YamlLoader;

pub struct Config {
    allowed_highways: HashMap<String, HashSet<String>>,
    max_speed: HashMap<String, u8>,
    default_walking_speed: u8,
}

impl Config {
    pub fn new() -> Config {
        let mut c = Config {
            allowed_highways: HashMap::new(),
            max_speed: HashMap::new(),
            default_walking_speed: 0,
        };

        c.init();
        return c;
    }

    pub fn init(&mut self) {
        let mut file = File::open("../../src/config.yaml").unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        let docs = YamlLoader::load_from_str(&file_content).unwrap();
        let doc = &docs[0];

        //
        let allowed_highways = doc["allowed_highways"].as_vec().unwrap();
        for entry in allowed_highways {
            for (key, types) in entry.as_hash().unwrap() {
                let types = types
                    .as_str()
                    .unwrap()
                    .split(", ")
                    .map(|x| x.to_string())
                    .collect::<HashSet<String>>();
                self.allowed_highways
                    .insert(key.as_str().unwrap().to_string(), types);
            }
        }

        //
        let max_speeds = doc["max_speed"].as_vec().unwrap();
        for entry in max_speeds {
            for (key, speed) in entry.as_hash().unwrap() {
                let speed = speed.as_i64().unwrap();
                let t = key.as_str().unwrap();
                self.max_speed.insert(t.to_string(), speed as u8);
            }
        }

        //
        self.default_walking_speed = doc["default_walking_speed"].as_i64().unwrap() as u8;
    }

    pub fn is_allowed(&self, key: &str, value: &str) -> bool {
        return self
            .allowed_highways
            .get(key)
            .unwrap_or(&HashSet::new())
            .contains(value);
    }

    pub fn default_walking_speed(&self) -> u8 {
        return self.default_walking_speed;
    }

    pub fn default_speed(&self, street_type: &String) -> u8 {
        return match self.max_speed.get(street_type) {
            Some(result) => *result,
            None => {
                println!("unknown street type: {}", street_type);
                *self.max_speed.get("unknown").unwrap()
            }
        };
    }
}
