extern crate osmpbfreader;
extern crate yaml_rust;

mod config;
mod graph;
mod osm_convert;
mod read_osm;
mod util;

use config::Config;


fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    match args.len() {
        2 => {
            let mut config = Config::new();

            let (nodes, ways) = read_osm::read_osm(&args[1], &config);
            let _graph = osm_convert::convert(nodes, ways, &config);
            //TODO:
            // - compute LLC
            // - proper argument parsing (llc, input file, network type)
            // - compute contraction
        }
        _ => println!("usage: filename",),
    };
}
