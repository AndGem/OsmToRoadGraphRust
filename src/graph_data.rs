pub struct NodeData {
    pub lat: f64,
    pub lon: f64,
}

pub struct EdgeData {
    pub name: String,
    pub street_type: String,
    pub max_speed: u8,
    pub bidirectional: bool,
}

pub trait NodeDataDescription {
    fn description(&self) -> String;
}

pub trait EdgeDataDescription {
    fn name(&self) -> String {
        "".to_string()
    }

    fn description(&self) -> String;
}

impl NodeDataDescription for NodeData {
    fn description(&self) -> String {
        format!("{:.6} {:.6}", self.lat, self.lon)
    }
}

impl EdgeDataDescription for EdgeData {
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn description(&self) -> String {
        let dir = if self.bidirectional { 1 } else { 0 };
        format!("{} {} {}", self.street_type, self.max_speed, dir)
    }
}
