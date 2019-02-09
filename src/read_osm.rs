use osmpbfreader::{Node, NodeId, Way};
use std::collections::HashMap;
use std::time::Instant;

use config;
use osmpbfreader;
use std;

fn filter_nodes_and_ways(
    mut nodes: HashMap<NodeId, Node>,
    mut ways: Vec<Way>,
    config: &config::Config,
) -> (HashMap<NodeId, Node>, Vec<Way>) {
    let mut ways_filtered: Vec<Way> = Vec::new();
    let mut nodes_filtered: HashMap<NodeId, Node> = HashMap::new();
    let nodes_initially = nodes.len();
    let ways_initially = ways.len();

    let now = Instant::now();
    while !ways.is_empty() {
        let way = ways.pop().unwrap();
        let all_nodes_available = way.nodes.iter().fold(true, |acc, x| {
            acc && (nodes.contains_key(x) || nodes_filtered.contains_key(x))
        });

        let is_area = way.tags.get("area").map(|x| x == "yes").unwrap_or(false);

        let allowed_highway = way
            .tags
            .get("highway")
            .map(|x| config.is_allowed("pedestrian", x))
            .unwrap_or(false);

        let way_ok = all_nodes_available && !is_area && allowed_highway;

        if way_ok {
            way.nodes
                .iter()
                .filter_map(|x| nodes.remove(x))
                .for_each(|n| {
                    nodes_filtered.insert(n.id, n);
                    ()
                });

            ways_filtered.push(way);
        }
    }

    println!(
        "filtered unnecessary nodes and ways: {}s",
        now.elapsed().as_secs()
    );
    println!(
        "#nodes now: {}/{} ({:.2}%)",
        nodes_filtered.len(),
        nodes_initially,
        nodes_filtered.len() as f64 / nodes_initially as f64 * 100.0
    );
    println!(
        "#ways now: {}/{} ({:.2}%)",
        ways_filtered.len(),
        ways_initially,
        ways_filtered.len() as f64 / ways_initially as f64 * 100.0
    );
    println!();

    return (nodes_filtered, ways_filtered);
}

fn read_nodes_and_ways(file_reference: std::fs::File) -> (HashMap<NodeId, Node>, Vec<Way>) {
    let mut pbf = osmpbfreader::OsmPbfReader::new(file_reference);

    let mut nodes = HashMap::new();
    let mut ways = Vec::new();

    let now = Instant::now();
    for obj in pbf.par_iter().map(Result::unwrap) {
        match obj {
            osmpbfreader::OsmObj::Node(node) => {
                nodes.insert(node.id, node);
            }
            osmpbfreader::OsmObj::Way(way) => {
                ways.push(way);
            }
            _ => {}
        }
    }

    println!("finished reading of osm data: {}s", now.elapsed().as_secs());
    println!(
        "data contains: {} ways, and {} nodes",
        ways.len(),
        nodes.len()
    );

    (nodes, ways)
}

pub fn read_osm(filename: &String, config: &config::Config) -> (HashMap<NodeId, Node>, Vec<Way>) {
    let file_reference = std::fs::File::open(&std::path::Path::new(filename)).unwrap();
    let (nodes, ways) = read_nodes_and_ways(file_reference);
    filter_nodes_and_ways(nodes, ways, config)
}
