use std::collections::HashMap;
use std::time::Instant;

use graph::Graph;
use graph_data::{EdgeData, NodeData};
use osm_parse_config;
use osmpbfreader::{Node, NodeId, Way};
use util;

pub fn convert(
    nodes: HashMap<NodeId, Node>,
    ways: Vec<Way>,
    osm_parse_config: &osm_parse_config::OSMParseConfig,
) -> Graph<NodeData, EdgeData> {
    let now = Instant::now();

    let mut g: Graph<NodeData, EdgeData> = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    let mut node_map = HashMap::new();

    for (k, v) in nodes {
        let data = NodeData {
            lat: v.lat(),
            lon: v.lon(),
        };

        let new_node_id = g.add_node(data);
        node_map.insert(k, new_node_id);
    }

    let empty_name = "".to_string();

    for way in ways {
        let name = way.tags.get("name").unwrap_or(&empty_name).to_string();
        let street_type = way.tags.get("highway").unwrap().to_string();
        let max_speed = parse_speed(way.tags.get("maxspeed"), &street_type, &osm_parse_config);
        let bidirectional = way.tags.get("oneway").map(|x| x != "yes").unwrap_or(true);

        let data = EdgeData {
            name: name,
            street_type: street_type,
            max_speed: max_speed,
            bidirectional: bidirectional,
        };

        let s = node_map.get(way.nodes.first().unwrap()).unwrap();
        let t = node_map.get(way.nodes.last().unwrap()).unwrap();

        match bidirectional {
            true => g.add_bidirectional_edge(s, t, data),
            false => g.add_unidirectional_edge(s, t, data),
        };
    }

    println!(
        "converted to a graph with {} edges and {} nodes: {}s",
        g.edges.len(),
        g.nodes.len(),
        now.elapsed().as_secs()
    );
    println!();

    g
}

fn parse_speed(
    speed: Option<&String>,
    street_type: &String,
    osm_parse_config: &osm_parse_config::OSMParseConfig,
) -> u8 {
    if speed.is_none() {
        return osm_parse_config.default_speed(street_type);
    }

    let speed_info = speed.unwrap();

    // try to convert it to u8; if valid return
    if speed_info.parse::<u8>().is_ok() {
        return speed_info.parse::<u8>().unwrap();
    }

    if speed_info.contains("walk") {
        return osm_parse_config.default_walking_speed();
    } else if speed_info.contains("none") {
        return osm_parse_config.default_speed(street_type);
    } else if speed_info.contains("mph") || speed_info.contains("mp/h") {
        let fac = 1.609344;
        let digits = util::keep_characters(speed_info, "0123456789");
        let value = digits.parse::<f64>().unwrap() * fac;
        return value as u8;
    } else if speed_info.contains("kph")
        || speed_info.contains("kmh")
        || speed_info.contains("km/h")
    {
        let digits = util::keep_characters(speed_info, "0123456789");
        let value = digits.parse::<u8>().unwrap();
        return value;
    } else {
        println!(
            "error while parsing max speed! Did not recognize: {}! Fallback used!",
            speed_info
        );
        return osm_parse_config.default_speed(street_type);
    }
}

// TESTS
#[cfg(test)]
use std::collections::HashSet;

#[test]
fn should_return_default_speed_for_highway_when_speed_is_none() {
    let (config, highway, highway_speed, _default_walking_speed) = create_config();

    let speed = None;
    let street_type: String = highway.to_owned();

    let result: u8 = parse_speed(speed, &street_type, &config);

    assert!(result == highway_speed);
}

#[test]
fn should_return_default_default_walking_speed_when_speed_is_walk() {
    let (config, highway, _highway_speed, default_walking_speed) = create_config();

    let speed_str = "walk".to_string();
    let speed = Some(&speed_str);
    let street_type: String = highway.to_owned();

    let result: u8 = parse_speed(speed, &street_type, &config);

    assert!(result == default_walking_speed);
}

#[test]
fn should_return_default_default_highway_speed_when_contains_none() {
    let (config, highway, highway_speed, _default_walking_speed) = create_config();

    let speed_str = "none".to_string();
    let speed = Some(&speed_str);
    let street_type: String = highway.to_owned();

    let result: u8 = parse_speed(speed, &street_type, &config);

    assert!(result == highway_speed);
}

#[test]
fn should_return_mph_speed() {
    let (config, highway, _highway_speed, _default_walking_speed) = create_config();

    let speed_str = "10 mph".to_string();
    let speed = Some(&speed_str);
    let street_type: String = highway.to_owned();

    let result: u8 = parse_speed(speed, &street_type, &config);
    let lower_bound = (10.0 * 1.60) as u8;
    let upper_bound = (10.0 * 1.61) as u8;

    assert!(result >= lower_bound);
    assert!(result <= upper_bound);
}

#[test]
fn should_return_kmh_speed() {
    let (config, highway, _highway_speed, _default_walking_speed) = create_config();

    let kmhs = ["kmh", "km/h", "kph"];
    let speed_str = "123";
    for s in kmhs.iter() {
        let speed_str = speed_str.to_string() + s;
        let speed = Some(&speed_str);
        let street_type: String = highway.to_owned();

        let result: u8 = parse_speed(speed, &street_type, &config);

        assert!(result == 123);
    }
}

#[test]
fn should_return_speed() {
    let (config, highway, _highway_speed, _default_walking_speed) = create_config();

    let speed_str = "22".to_string();
    let speed = Some(&speed_str);
    let street_type: String = highway.to_owned();

    let result: u8 = parse_speed(speed, &street_type, &config);

    assert!(result == 22);
}

#[test]
fn should_return_default_speed_if_garbage() {
    let (config, highway, highway_speed, _default_walking_speed) = create_config();

    let speed_str = "garbage".to_string();
    let speed = Some(&speed_str);
    let street_type: String = highway.to_owned();

    let result: u8 = parse_speed(speed, &street_type, &config);

    assert!(result == highway_speed);
}

#[cfg(test)]
fn create_config() -> (osm_parse_config::OSMParseConfig, String, u8, u8) {
    let highway: String = "barfoo".to_string();
    let highway_speed: u8 = 23;
    //
    let mut max_speed: HashMap<String, u8> = HashMap::new();
    max_speed.insert(highway.to_owned(), highway_speed);
    //
    let default_walking_speed: u8 = 12;
    //
    let mut allowed_highways = HashSet::new();
    allowed_highways.insert(highway.to_owned());

    let config =
        osm_parse_config::OSMParseConfig::new(HashMap::new(), max_speed, default_walking_speed);

    return (config, highway, highway_speed, default_walking_speed);
}
