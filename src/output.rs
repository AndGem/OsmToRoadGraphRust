use graph;

use graph::{EdgeDataAccess, GraphEdgeFormat, NodeDataAccess};
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn write<TN: NodeDataAccess, TE: EdgeDataAccess>(
    g: &graph::Graph<TN, TE>,
    filename: String,
) -> Result<(), io::Error> {
    let mut f = File::create(filename)?;
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

pub fn write_names<TN: NodeDataAccess, TE: EdgeDataAccess>(
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
