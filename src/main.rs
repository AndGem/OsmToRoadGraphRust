extern crate osmpbfreader;
extern crate yaml_rust;

mod config;
mod config_creator;
mod graph;
mod osm_convert;
mod output;
mod read_osm;
mod util;


fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    match args.len() {
        2 => {
            let filename = "../../src/config.yaml";
            let config = config_creator::create_config_from_file(filename.to_string());

            let (nodes, ways) = read_osm::read_osm(&args[1], &config);
            let graph = osm_convert::convert(nodes, ways, &config);
            let r = output::write(graph);
            match r {
                Ok(_) => (),
                Err(y) => println!("ERROR: {}", y),
            };
            //TODO:
            // - compute LLC
            // - proper argument parsing (llc, input file, network type)
            // - compute contraction
            // - output
            // - what is the best way to integrate the configuration yaml file?
            // - 64bit prec for float necessary?
        }
        _ => println!("usage: filename",),
    };
}
