#[macro_use]
extern crate clap;
extern crate osmpbfreader;
#[macro_use]
extern crate lazy_static;
extern crate yaml_rust;
use clap::App;
use network_type::{get_network_type, NetworkType};

mod graph;
mod graph_data;
mod network_type;
mod osm_convert;
mod osm_parse_config;
mod osm_parse_config_creator;
mod osm_reader;
mod output;
mod util;

lazy_static! {
    static ref VERBOSE: bool = {
        let yaml = load_yaml!("cli.yaml");
        let arg_matches = App::from_yaml(yaml).get_matches();

        arg_matches.is_present("verbose")
    };
    static ref NETWORK_TYPE: NetworkType = {
        let yaml = load_yaml!("cli.yaml");
        let arg_matches = App::from_yaml(yaml).get_matches();

        let network_type = get_network_type(arg_matches.value_of("network").unwrap());
        println!("converting OSM to network_type: {:?}", network_type);
        network_type
    };
    static ref NO_LLC: bool = {
        let yaml = load_yaml!("cli.yaml");
        let arg_matches = App::from_yaml(yaml).get_matches();

        arg_matches.is_present("nolcc")
    };
    static ref CONTRACT: bool = {
        let yaml = load_yaml!("cli.yaml");
        let arg_matches = App::from_yaml(yaml).get_matches();

        arg_matches.is_present("contract")
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

    //process
    let in_filename = arg_matches.value_of("input").unwrap();
    let (nodes, ways) = osm_reader::read_osm(&in_filename.to_owned(), &config);
    let graph = osm_convert::convert(nodes, ways, &config);

    //output
    let out_filename = create_out_filename(in_filename, &(NETWORK_TYPE));
    let out_filename_names = format!("{}_names", out_filename);

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
    // - compute LCC
    // - compute contraction
    // - add code coverage
    // - fill README.MD
    // - add to travis a test case to convert an OSM file
}

fn create_out_filename(in_filename: &str, network_type: &NetworkType) -> String {
    let identifier = match network_type {
        NetworkType::Pedestrian => "p",
        NetworkType::Car => "c",
        NetworkType::Bicycle => "b",
    };

    format!("{}.py{}gr", in_filename, identifier)
}
