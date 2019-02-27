#[derive(PartialEq, Eq, Hash)]
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

pub fn short_network_type(t: &NetworkType) -> &str {
    match t {
        Pedestrian => "p",
        Car => "c",
        Bicycle => "b",
    }
}
