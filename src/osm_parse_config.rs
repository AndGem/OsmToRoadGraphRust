use std::collections::{HashMap, HashSet};

pub struct OSMParseConfig {
    allowed_highways: HashMap<::NetworkType, HashSet<String>>,
    max_speed: HashMap<String, u8>,
    default_walking_speed: u8,
}

impl OSMParseConfig {
    pub fn new(
        allowed_highways: HashMap<::NetworkType, HashSet<String>>,
        max_speed: HashMap<String, u8>,
        default_walking_speed: u8,
    ) -> OSMParseConfig {
        OSMParseConfig {
            allowed_highways: allowed_highways,
            max_speed: max_speed,
            default_walking_speed: default_walking_speed,
        }
    }

    pub fn is_allowed(&self, key: &::NetworkType, value: &str) -> bool {
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
