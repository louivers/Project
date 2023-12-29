use crate::models::query::{Query, DataBase, NaturalJoin};
use crate::algorithms::helper::{find_root, post_order_apply};
use crate::algorithms::join_tree::generate_join_tree;
use crate::algorithms::full_reducer::globally_consistent_database;
use crate::algorithms::joins::naturaljoin;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn yannakakis(query:Query, database: &mut DataBase) {
    // build a join tree from the query
    let join_tree = generate_join_tree(&query.body).unwrap();

    // find a root in the join tree
    let root = find_root(&join_tree).unwrap();

    // build a globally consistent database
    globally_consistent_database(database, &join_tree);

    // build a full reducer style, but with joins instead of semijoins
    let mut joins: Vec<NaturalJoin> = Vec::new();

    // bottom up traversal of the join tree to build first joins of the reducer
    post_order_apply(&join_tree, petgraph::graph::NodeIndex::new(root), &mut |join_tree, node| {
        // take the node from the join tree and find its parent
        let parent = join_tree
            .neighbors_directed(node, petgraph::Direction::Incoming)
            .next();
        // if the node has no parent, it is the root of the join tree 
        // nothing needs to be added to the reducer
        if parent == None {
            return;
        } else {
            // if the node has a parent, add a join to the reducer
            // the join has the node as its right child and the parent as its left child
            let join = NaturalJoin{
                left: join_tree[node].to_owned(),
                right: join_tree[parent.unwrap()].to_owned(),
            };
            joins.push(join);
        }
    });
    // print the joins
    for join in &joins {
        println!("{}", join);
    }

    // perform the joins
    for join in joins {
        naturaljoin(join, database, query.head.clone());
    }

}