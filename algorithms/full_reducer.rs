use crate::models::query::{Atom, SemiJoin, DataBase};
use petgraph::{Graph, Directed};
use crate::algorithms::helper::{find_root, post_order_apply, pre_order_apply};
use crate::algorithms::joins::semijoin;

#[allow(dead_code)]
pub fn globally_consistent_database(database: &mut DataBase, join_tree: &Graph<Atom, u8, Directed>) {
    let full_reducer = build_full_reducer_from_tree(join_tree);
    for semij in full_reducer {

        semijoin(&semij, database);
    }
}   

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn build_full_reducer_from_tree(join_tree: &Graph<Atom, u8, Directed>) -> Vec<SemiJoin> {
    // the full reducer is a vector of SemiJoins
    let mut reducer = Vec::new();
    // find a root in the join tree
    let mut root = find_root(join_tree).unwrap();
    // bottom up traversal of the join tree to build first semijoins of the reducer
    post_order_apply(join_tree, petgraph::graph::NodeIndex::new(root), &mut |join_tree, node| {
        // take the node from the join tree and find its parent
        let parent = join_tree
            .neighbors_directed(node, petgraph::Direction::Incoming)
            .next();
        // if the node has no parent, it is the root of the join tree 
        // nothing needs to be added to the reducer
        if parent == None {
            return;
        } else {
            // if the node has a parent, add a semijoin to the reducer
            // the semijoin has the node as its right child and the parent as its left child
            let semijoin = SemiJoin{
                right: join_tree[node].to_owned(),
                left: join_tree[parent.unwrap()].to_owned(),
            };
            reducer.push(semijoin);
        }
    });
    
    // top down traversal of the join tree to build the rest of the reducer
    pre_order_apply(join_tree, petgraph::graph::NodeIndex::new(root), &mut |join_tree, node| {
        // take the node from the join tree and find its children
        let children = join_tree
            .neighbors_directed(node, petgraph::Direction::Outgoing)
            .collect::<Vec<_>>();
        // if the node has no children, it is a leaf of the join tree
        // nothing needs to be added to the reducer
        if children.len() == 0 {
            return;
        } else {
            // if the node has children, add a semijoin to the reducer for each direct child
            // the semijoin has the node as its right and the first child as its left
            for child in children.iter() {
                let semijoin = SemiJoin{
                    right: join_tree[node].to_owned(),
                    left: join_tree[*child].to_owned(),
                };
                reducer.push(semijoin);
            }
            
        }
    });
    return reducer;
}