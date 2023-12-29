use crate::models::query::Atom;
use petgraph::{Graph, Directed};

// Returns the index of the root of the given join tree.
pub fn find_root(join_tree: &Graph<Atom, u8, Directed>) -> Option<usize> {
    for node in join_tree.node_indices() {
        if join_tree.neighbors_directed(node, petgraph::Direction::Incoming).count() == 0 {
            return Some(node.index());
        }
    }
    return None;
}

pub fn post_order_apply<F>(
    join_tree: &Graph<Atom, u8, Directed>,
    root: petgraph::graph::NodeIndex,
    funct: &mut F)
    where F: FnMut(&Graph<Atom, u8, Directed>, petgraph::graph::NodeIndex) {
    let mut stack = Vec::new();
    let mut visited = vec![false; join_tree.node_count()];
    stack.push(root);
    while !stack.is_empty() {
        let node = *stack.last().unwrap();
        if !visited[node.index()] {
            for neighbor in join_tree.neighbors_directed(node, petgraph::Direction::Outgoing) {
                stack.push(neighbor);
            }
            visited[node.index()] = true;
        } else {
            stack.pop();
            funct(join_tree, node);
        }
    }
}

pub fn pre_order_apply<F>(
    join_tree: &Graph<Atom, u8, Directed>,
    root: petgraph::graph::NodeIndex,
    funct: &mut F)
    where F: FnMut(&Graph<Atom, u8, Directed>, petgraph::graph::NodeIndex) {
    let mut stack = Vec::new();
    stack.push(root);
    while stack.len() != 0 {
        let node = stack.pop().unwrap();
        funct(join_tree, node);
        for neighbor in join_tree.neighbors_directed(node, petgraph::Direction::Outgoing) {
            stack.push(neighbor);
        }
    }
}