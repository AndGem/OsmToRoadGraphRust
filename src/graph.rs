use graph_data::EdgeDataDescription;

#[derive(Debug, Copy, Clone)]
pub struct GraphNodeId(pub u32);

#[derive(Debug, Copy, Clone)]
pub struct GraphEdgeId(pub u32);

pub struct GraphNode<T> {
    pub id: GraphNodeId,
    pub data: T,
    out_edges: Vec<GraphEdgeId>,
}

pub struct GraphEdge<T> {
    pub id: GraphEdgeId,
    pub s: GraphNodeId,
    pub t: GraphNodeId,
    pub data: T,
}

pub trait GraphEdgeDescription {
    fn description(&self) -> String;
}

pub struct Graph<NodeData, EdgeData> {
    pub nodes: Vec<GraphNode<NodeData>>,
    pub edges: Vec<GraphEdge<EdgeData>>,
}

impl<T> GraphNode<T> {
    pub fn add_edge(&mut self, id: GraphEdgeId) {
        self.out_edges.push(id);
    }

    pub fn get_edges(&self) -> &Vec<GraphEdgeId> {
        &self.out_edges
    }
}

impl<T: EdgeDataDescription> GraphEdgeDescription for GraphEdge<T> {
    fn description(&self) -> String {
        format!("{} {} {}", self.s.0, self.t.0, self.data.description())
    }
}

impl<NodeData, EdgeData> Graph<NodeData, EdgeData> {
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
        s: GraphNodeId,
        t: GraphNodeId,
        edge_data: EdgeData,
        bidirectional: bool,
    ) {
        let edge_index = GraphEdgeId(self.edges.len() as u32);

        let new_edge = GraphEdge {
            id: edge_index,
            s,
            t,
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
