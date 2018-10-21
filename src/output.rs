use graph;

use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::fmt;


pub fn write<TN: fmt::Display, TE: fmt::Display>(g: graph::Graph<TN, TE>) -> Result<(), io::Error> {
    let mut f = File::create("foo.txt")?;
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
