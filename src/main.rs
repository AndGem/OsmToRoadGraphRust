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
    let mut args: Vec<_> = std::env::args_os().collect();
    match args.len() {
        2 => {
            let filename = "../../src/config.yaml";
            let config = config_creator::create_config_from_file(filename.to_string());

            let in_filename = args.remove(1).into_string().unwrap();

            let (nodes, ways) = read_osm::read_osm(&in_filename.to_owned(), &config);
            let graph = osm_convert::convert(nodes, ways, &config);
            let r = output::write(graph, in_filename.to_owned() + ".out");
            match r {
                Ok(_) => (),
                Err(y) => println!("ERROR: {}", y),
            };
            //TODO:
            // - compute LLC
            // - proper argument parsing (llc, input file, network type)
            // - compute contraction
            // - output filename depending on the pedestrian type
            // - what is the best way to integrate the configuration yaml file?
            // - 64bit prec for float necessary?
        }
        _ => println!("usage: filename",),
    };
}
