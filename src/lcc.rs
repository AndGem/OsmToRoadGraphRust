use graph::{Graph, GraphEdge, GraphNode, GraphNodeId};
use graph_data::{EdgeData, NodeData};

use std::collections::VecDeque;

pub fn compute_lcc(graph: &Graph<NodeData, EdgeData>) {}

fn kosaraju(graph: &Graph<NodeData, EdgeData>) {
    //TODO: remove implicit assumption that node indices are integers from 0 to n - 1
    //TODO: remove all leakage that indices are u32

    let mut L: VecDeque<&GraphNodeId> = kosaraju_step1_2(graph);

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

fn kosaraju_step1_2(graph: &Graph<NodeData, EdgeData>) -> VecDeque<&GraphNodeId> {
    // 1. For each vertex u of the graph, mark u as unvisited. Let list L be empty.
    // 2. For each vertex u of the graph do Visit(u), where Visit(u) is the recursive subroutine:
    let mut visited = vec![false; graph.node_count() as usize];
    let mut L: VecDeque<&GraphNodeId> = VecDeque::new();

    let mut queue: VecDeque<u32> = VecDeque::new();
    let mut node_index = 0;

    while node_index < visited.len() {
        if visited[node_index] {
            node_index += 1;
            continue;
        }

        queue.push_back(node_index as u32);
        visited[node_index] = true;

        while !queue.is_empty() {
            let node_id = queue.pop_front().unwrap();
            let out_edges = graph.out_edges(node_id);
            visited[node_id as usize] = true;

            //TODO: construct in-edges on the fly
            for e in out_edges.iter() {
                let edge_data = graph.get_edge(e);
                let t = edge_data.t.0; //TODO: max access nicer
                if visited[t as usize] {
                    continue;
                }

                L.push_back(&edge_data.t);
                queue.push_back(t);
            }
        }
    }
    L //TODO: return in-edges
}

//todo test kosaraju_step12
