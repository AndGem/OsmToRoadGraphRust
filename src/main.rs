#[macro_use]
extern crate clap;
extern crate osmpbfreader;
#[macro_use]
extern crate lazy_static;
extern crate yaml_rust;
use clap::App;

mod graph;
mod graph_data;
mod osm_convert;
mod osm_parse_config;
mod osm_parse_config_creator;
mod osm_reader;
mod output;
mod util;

//TODO: use this block to initialize a configuration object
lazy_static! {
    static ref VERBOSE: bool = {
        let yaml = load_yaml!("cli.yaml");
        let arg_matches = App::from_yaml(yaml).get_matches();
        arg_matches.is_present("verbose")
    };
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let arg_matches = App::from_yaml(yaml).get_matches();

    let default_config_str = include_str!("config.yaml");
    let default_config =
        osm_parse_config_creator::create_config_from_string(default_config_str.to_owned());
    let config = arg_matches
        .value_of("config")
        .map_or(default_config, |input_file| {
            osm_parse_config_creator::create_config_from_file(input_file.to_owned())
        });
    let network_type = arg_matches.value_of("network").unwrap();
    let no_llc = arg_matches.is_present("nollc");
    let contract = arg_matches.is_present("contract");

    //process
    let in_filename = arg_matches.value_of("input").unwrap();
    let (nodes, ways) = osm_reader::read_osm(&in_filename.to_owned(), &config);
    let graph = osm_convert::convert(nodes, ways, &config);

    //output
    let out_filename = format!("{}.py{}gr", in_filename, network_type);
    let out_filename_names = format!("{}.py{}gr_names", in_filename, network_type);
    println!("writing graph to {}", out_filename);
    let output_result = output::write(&graph, out_filename);
    match output_result {
        Ok(_) => (),
        Err(y) => println!("ERROR: {}", y),
    };

    println!("writing street names to {}", out_filename_names);
    let output_result = output::write_names(&graph, out_filename_names);
    match output_result {
        Ok(_) => (),
        Err(y) => println!("ERROR: {}", y),
    };

    //TODO:
    // - compute LLC
    // - compute contraction
    // - add code coverage
    // - fill README.MD
}
