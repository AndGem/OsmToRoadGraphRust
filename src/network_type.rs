#[derive(PartialEq, Eq, Hash, Debug)]
pub enum NetworkType {
    Pedestrian,
    Car,
    Bicycle,
}

pub fn get_network_type(network_type: &str) -> NetworkType {
    match network_type {
        "pedestrian" | "p" => NetworkType::Pedestrian,
        "car" | "c" => NetworkType::Car,
        "bicycle" | "b" => NetworkType::Bicycle,
        _ => panic!("did not recognize type: {}", network_type),
    }
}
