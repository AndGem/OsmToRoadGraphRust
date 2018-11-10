use graph;

use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn write<TN: fmt::Display, TE: fmt::Display>(
    g: graph::Graph<TN, TE>,
    filename: String,
) -> Result<(), io::Error> {
    let mut f = File::create(filename)?;
    for node in g.nodes {
        f.write(node.to_string().as_bytes())?;
        f.write(b"\n")?;
    }

    for edge in g.edges {
        f.write(edge.to_string().as_bytes())?;
        f.write(b"\n")?;
    }

    Ok(())
}
