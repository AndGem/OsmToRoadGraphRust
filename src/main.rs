extern crate osmpbfreader;
extern crate yaml_rust;
extern crate clap;
use clap::{Arg, App};

mod config;
mod config_creator;
mod graph;
mod osm_convert;
mod output;
mod read_osm;
mod util;


fn main() {
    let matches = App::new("OSM to Road Graph")
                        .version("0.1")
                        .about("Converts OSM PBF files to a simple graph format.")
                        .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .value_name("FILE")
                            .help("Sets a custom config file")
                            .takes_value(true))
                        .arg(Arg::with_name("OSM .pbf file")
                            .help("Sets the input osm pbf file to use")
                            .required(true)
                            .index(1))
                        .arg(Arg::with_name("network")
                            .help("sets the network type: (p)edestrian, (b)icycle, (c)ar")
                            .short("-n")
                            .long("networkType")
                            .default_value("p")
                            .takes_value(true))
                        .arg(Arg::with_name("nollc")
                            .help("do not compute only largest connected component")
                            .short("-l")
                            .long("nollc"))
                        .arg(Arg::with_name("contract")
                            .help("compute also graph with contracted deg 2 nodes")
                            .short("-c")
                            .long("contract"))
                        .get_matches();


    let default_config_str = include_str!("config.yaml");
    let default_config = config_creator::create_config_from_string(default_config_str.to_owned());
    let config = matches.value_of("config")
                        .map_or(default_config, |input_file| config_creator::create_config_from_file(input_file.to_owned()));
    let network_type = matches.value_of("network");
    let no_llc = matches.is_present("nollc");
    let contract = matches.is_present("contract");

    let in_filename =  matches.value_of("OSM .pbf file").unwrap();

    let (nodes, ways) = read_osm::read_osm(&in_filename.to_owned(), &config);
    let graph = osm_convert::convert(nodes, ways, &config);
    let r = output::write(graph, in_filename.to_owned() + ".out");
    match r {
        Ok(_) => (),
        Err(y) => println!("ERROR: {}", y),
    };

    //TODO:
    // - compute LLC
    // - output filename depending on the network type
    // - compute contraction
    // - 64bit prec for float necessary?
    // - add code coverage
    // - add clap config to external yaml
}
