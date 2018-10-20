use std::collections::HashMap;

use config;
use graph::{EdgeData, Graph, NodeData};
use osmpbfreader::{Node, NodeId, Way};
use util;

pub fn convert(
    nodes: HashMap<NodeId, Node>,
    ways: Vec<Way>,
    config: &config::Config,
) -> Graph<NodeData, EdgeData> {
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
        let max_speed = parse_speed(way.tags.get("maxspeed"), &street_type, &config);
        let bidirectional = way.tags.get("oneway").map(|x| x != "yes").unwrap_or(true);

        let data = EdgeData {
            name: name,
            street_type: street_type,
            max_speed: max_speed,
            bidirectional: bidirectional,
        };

        let s = node_map.get(way.nodes.first().unwrap()).unwrap();
        let t = node_map.get(way.nodes.last().unwrap()).unwrap();

        g.add_edge(s, t, data, bidirectional);
    }

    println!("{} edges", g.edges.len());
    println!("{} nodes", g.nodes.len());
    g
}

fn parse_speed(speed: Option<&String>, street_type: &String, config: &config::Config,) -> u8 {
    if speed.is_none() {
        return config.default_speed(street_type);
    }

    let speed_info = speed.unwrap();

    // try to convert it to u8; if valid return
    if speed_info.parse::<u8>().is_ok() {
        return speed_info.parse::<u8>().unwrap();
    }

    if speed_info.contains("walk") {
        return config.default_walking_speed();
    } else if speed_info.contains("none") {
        return config.default_speed(street_type);
    } else if speed_info.contains("mph") || speed_info.contains("mp/h") {
        let fac = 1.609344;
        let digits = util::keep_characters(speed_info, "0123456789");
        let value = digits.parse::<f64>().unwrap() * fac;
        return value as u8;
    } else if speed_info.contains("kph")
        || speed_info.contains("kp/h")
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
        return config.default_speed(street_type);
    }
}
