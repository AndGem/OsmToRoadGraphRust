use graph::{Graph, GraphEdgeId, GraphNodeId};
use graph_data::{EdgeData, NodeData};

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn compute_lcc(graph: &Graph<NodeData, EdgeData>) {
    let _x = kosaraju(graph);
}

fn kosaraju(graph: &Graph<NodeData, EdgeData>) {
    //TODO: remove implicit assumption that node indices are integers from 0 to n - 1

    let (_result, _in_edges) = kosaraju_step1_2(graph);

    // 3. For each element u of L in order, do Assign(u,u) where Assign(u,root) is the recursive subroutine:

    // From: https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm
    // 1. For each vertex u of the graph, mark u as unvisited. Let list L be empty.
    // 2. For each vertex u of the graph do Visit(u), where Visit(u) is the recursive subroutine:
    // If u is unvisited then:
    //      Mark u as visited.
    //      For each out-neighbour v of u, do Visit(v).
    //      Prepend u to L.
    // Otherwise do nothing.

    // 3. For each element u of L in order, do Assign(u,u) where Assign(u,root) is the recursive subroutine:
    // If u has not been assigned to a component then:
    //      Assign u as belonging to the component whose root is root.
    //      For each in-neighbour v of u, do Assign(v,root).
    // Otherwise do nothing.
}

fn kosaraju_step1_2<S, T>(
    graph: &Graph<S, T>,
) -> (
    VecDeque<GraphNodeId>,
    HashMap<GraphNodeId, Vec<GraphEdgeId>>,
) {
    // 1. For each vertex u of the graph, mark u as unvisited. Let list L be empty.
    // 2. For each vertex u of the graph do Visit(u), where Visit(u) is the recursive subroutine:
    let mut ordering: VecDeque<GraphNodeId> = VecDeque::new();
    let mut in_edges: HashMap<GraphNodeId, Vec<GraphEdgeId>> = HashMap::new();
    let nodes = graph.node_indices();
    let mut visited = HashSet::new();

    let mut queue: VecDeque<GraphNodeId> = VecDeque::new();
    for start_node_id in nodes {
        if visited.contains(&start_node_id) {
            continue;
        }

        queue.push_back(start_node_id);

        while !queue.is_empty() {
            let node_id = queue.pop_front().unwrap();
            if visited.contains(&node_id) {
                continue;
            }

            visited.insert(node_id);

            let out_edges = graph.out_edges(&node_id);
            for edge_id in out_edges.iter() {
                let edge_data = graph.get_edge(edge_id);
                queue.push_back(edge_data.t);

                match in_edges.entry(edge_data.t) {
                    Entry::Vacant(entry) => {
                        entry.insert(vec![*edge_id]);
                    }
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(*edge_id);
                    }
                }
            }
            println!();
            ordering.push_back(node_id);
        }
    }

    (ordering, in_edges)
}

// TESTS
#[cfg(test)]
use galvanic_assert::matchers::collection::*;
use graph::NothingData;

#[test]
fn graph_with_two_nodes_should_return_both() {
    let mut g: Graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    let s = g.add_node(NothingData {});
    let t = g.add_node(NothingData {});
    g.add_bidirectional_edge(&s, &t, NothingData {});

    let (result, in_edges) = kosaraju_step1_2(&g);
    assert_that!(
        &result,
        contains_in_order(vec![GraphNodeId(0), GraphNodeId(1)])
    );
}

#[test]
fn graph_with_four_nodes_and_cycle() {
    let mut g: Graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    // +-+    0   +-+   1    +-+
    // |0+------->+1+------->+2|
    // +++        +++        +-+
    //  ^          | 2
    //  |          v
    //  |    3    +++
    //  +---------+3|
    //            +-+

    let v0 = g.add_node(NothingData {});
    let v1 = g.add_node(NothingData {});
    let v2 = g.add_node(NothingData {});
    let v3 = g.add_node(NothingData {});
    let e0 = g.add_unidirectional_edge(&v0, &v1, NothingData {});
    let e1 = g.add_unidirectional_edge(&v1, &v2, NothingData {});
    let e2 = g.add_unidirectional_edge(&v1, &v3, NothingData {});
    let e3 = g.add_unidirectional_edge(&v3, &v0, NothingData {});

    let (result, in_edges) = kosaraju_step1_2(&g);
    assert_that!(
        &result,
        contains_in_order(vec![
            GraphNodeId(0),
            GraphNodeId(1),
            GraphNodeId(2),
            GraphNodeId(3)
        ])
    );

    assert_that!(in_edges.get(&v0).unwrap(), contains_in_order(vec![e3]));
    assert_that!(in_edges.get(&v1).unwrap(), contains_in_order(vec![e0]));
    assert_that!(in_edges.get(&v2).unwrap(), contains_in_order(vec![e1]));
    assert_that!(in_edges.get(&v3).unwrap(), contains_in_order(vec![e2]));
}

#[test]
fn graph_with_disconnected_component() {
    let mut g: Graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    // +-+   0    +-+   1    +-+
    // |0+------->+2+------->+3|
    // +++        +++        +-+
    //             |2
    //             v
    //            +++  +++
    //            +4|  +1|
    //            +-+  +-+

    let v0 = g.add_node(NothingData {});
    let v1 = g.add_node(NothingData {});
    let v2 = g.add_node(NothingData {});
    let v3 = g.add_node(NothingData {});
    let v4 = g.add_node(NothingData {});
    let e0 = g.add_unidirectional_edge(&v0, &v2, NothingData {});
    let e1 = g.add_unidirectional_edge(&v2, &v3, NothingData {});
    let e2 = g.add_unidirectional_edge(&v2, &v4, NothingData {});

    let (result, in_edges) = kosaraju_step1_2(&g);
    assert_that!(
        &result,
        contains_in_order(vec![
            GraphNodeId(0),
            GraphNodeId(2),
            GraphNodeId(3),
            GraphNodeId(4),
            GraphNodeId(1)
        ])
    );
    assert!(in_edges.get(&v0).is_none());
    assert!(in_edges.get(&v1).is_none());

    assert_that!(in_edges.get(&v2).unwrap(), contains_in_order(vec![e0]));
    assert_that!(in_edges.get(&v3).unwrap(), contains_in_order(vec![e1]));
    assert_that!(in_edges.get(&v4).unwrap(), contains_in_order(vec![e2]));
}

#[test]
fn empty_graph_returns_empty_response() {
    let g: Graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    let (result, in_edges) = kosaraju_step1_2(&g);

    assert_eq!(result.len(), 0);
    assert_eq!(in_edges.len(), 0);
}
