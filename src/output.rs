use graph;

use graph::GraphEdgeDescription;
use graph_data::{EdgeDataDescription, NodeDataDescription};

use std::fs::File;
use std::io;
use std::io::prelude::*;

const HEADER: &str = "# Road Graph File v.0.4\
                      # number of nodes\
                      # number of edges \
                      # node_properties \
                      # ... \
                      # edge_properties \
                      # ...";

pub fn write<TN: NodeDataDescription, TE: EdgeDataDescription>(
    g: &graph::Graph<TN, TE>,
    filename: String,
) -> Result<(), io::Error> {
    let mut f = File::create(filename)?;

    f.write(HEADER.as_bytes())?;
    f.write(b"\n")?;

    for node in &g.nodes {
        f.write(node.data.description().as_bytes())?;
        f.write(b"\n")?;
    }

    for edge in &g.edges {
        f.write(edge.description().as_bytes())?;
        f.write(b"\n")?;
    }

    Ok(())
}

pub fn write_names<TN: NodeDataDescription, TE: EdgeDataDescription>(
    g: &graph::Graph<TN, TE>,
    filename: String,
) -> Result<(), io::Error> {
    let mut f = File::create(filename)?;

    for edge in &g.edges {
        f.write(edge.data.name().as_bytes())?;
        f.write(b"\n")?;
    }

    Ok(())
}
