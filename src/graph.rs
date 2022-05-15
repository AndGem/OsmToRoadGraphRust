use graph_data::EdgeDataDescription;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct GraphNodeId(pub u32);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct GraphEdgeId(pub u32);

pub struct NothingData;

pub struct GraphNode<T = NothingData> {
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

pub struct Graph<NodeData = NothingData, EdgeData = NothingData> {
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

    pub fn add_unidirectional_edge(
        &mut self,
        s: &GraphNodeId,
        t: &GraphNodeId,
        edge_data: EdgeData,
    ) -> GraphEdgeId {
        self.add_edge(*s, *t, edge_data, false)
    }

    pub fn add_bidirectional_edge(
        &mut self,
        s: &GraphNodeId,
        t: &GraphNodeId,
        edge_data: EdgeData,
    ) -> GraphEdgeId {
        self.add_edge(*s, *t, edge_data, true)
    }

    fn add_edge(
        &mut self,
        s: GraphNodeId,
        t: GraphNodeId,
        edge_data: EdgeData,
        bidirectional: bool,
    ) -> GraphEdgeId {
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

        edge_index
    }

    pub fn node_indices(&self) -> Vec<GraphNodeId> {
        if self.nodes.is_empty() {
            return Vec::new();
        }

        let last_node_id = self.nodes.len() as u32;
        let result = (0..last_node_id).map(|x| GraphNodeId(x)).collect();
        result
    }

    pub fn edge_count(&self) -> u32 {
        self.edges.len() as u32
    }

    pub fn node_count(&self) -> u32 {
        self.nodes.len() as u32
    }

    pub fn out_edges(&self, index: &GraphNodeId) -> &Vec<GraphEdgeId> {
        self.nodes[index.0 as usize].get_edges()
    }

    pub fn get_edge(&self, id: &GraphEdgeId) -> &GraphEdge<EdgeData> {
        &self.edges[id.0 as usize]
    }
}

/*
 - TODO: remove all leakage that indices are u32
*/
