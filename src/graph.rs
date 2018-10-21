use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct GraphNodeId(pub u32);

#[derive(Debug, Copy, Clone)]
pub struct GraphEdgeId(pub u32);

pub struct NodeData {
    pub lat: f64, //TODO: not sure 64bit is necessary for precision here (and for lon)
    pub lon: f64,
}

impl fmt::Display for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.6} {:.6}", self.lat, self.lon)
    }
}

pub struct EdgeData {
    pub name: String,
    pub street_type: String,
    pub max_speed: u8,
    pub bidirectional: bool,
}

impl fmt::Display for EdgeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir = if self.bidirectional { 1 } else { 0 };
        write!(f, "{} {} {}", self.street_type, self.max_speed, dir)
    }
}

pub struct GraphNode<T> {
    pub id: GraphNodeId,
    pub data: T,
    out_edges: Vec<GraphEdgeId>,
}

impl<T: fmt::Display> fmt::Display for GraphNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id.0, self.data.to_string())
    }
}

impl<T: fmt::Display> GraphNode<T> {
    pub fn add_edge(&mut self, id: GraphEdgeId) {
        self.out_edges.push(id);
    }

    pub fn get_edges(&self) -> &Vec<GraphEdgeId> {
        &self.out_edges
    }
}

pub struct GraphEdge<T> {
    pub id: GraphEdgeId,
    pub s: GraphNodeId,
    pub t: GraphNodeId,
    pub data: T,
}

impl<T: fmt::Display> fmt::Display for GraphEdge<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.s.0, self.t.0, self.data.to_string())
    }
}

pub struct Graph<NodeData: fmt::Display, EdgeData: fmt::Display> {
    pub nodes: Vec<GraphNode<NodeData>>,
    pub edges: Vec<GraphEdge<EdgeData>>,
}

impl<NodeData: fmt::Display, EdgeData: fmt::Display> Graph<NodeData, EdgeData> {

    pub fn add_node(&mut self, node_data: NodeData) -> GraphNodeId {
        let node_id = GraphNodeId(self.nodes.len() as u32);
        let new_node = GraphNode {
            id: node_id,
            data: node_data,
            out_edges: Vec::new(),
        };
        self.nodes.push(new_node);
        node_id
    }

    pub fn add_edge(
        &mut self,
        s: &GraphNodeId,
        t: &GraphNodeId,
        edge_data: EdgeData,
        bidirectional: bool,
    ) {
        let edge_index = GraphEdgeId(self.edges.len() as u32);

        let new_edge = GraphEdge {
            id: edge_index,
            s: *s,
            t: *t,
            data: edge_data,
        };
        self.edges.push(new_edge);

        self.nodes
            .get_mut(s.0 as usize)
            .unwrap()
            .add_edge(edge_index);

        if bidirectional {
            self.nodes
                .get_mut(t.0 as usize)
                .unwrap()
                .add_edge(edge_index);
        }
    }
}
