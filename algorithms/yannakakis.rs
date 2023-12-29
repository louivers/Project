use crate::models::query::{Query, DataBase, NaturalJoin, Term};
use crate::algorithms::helper::{find_root, post_order_apply};
use crate::algorithms::join_tree::generate_join_tree;
use crate::algorithms::full_reducer::globally_consistent_database;
use crate::algorithms::joins::naturaljoin;

use super::gyo;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn yannakakis(query:Query, database: &mut DataBase) {
    //verify that the query is acyclic
    let acyclic = gyo::gyo(&query);
    // if the query is not acyclic, give an error
    if !acyclic {
        panic!("The query is not acyclic");
    }

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

    
    // find the attributes that the variables in the head of the query are bound to
    let mut projectionattributes: Vec<String> = Vec::new();
    // look at all the variables in the head of the query
    for var in &query.head{
        // look at all the atoms in the body of the query
        for atom in &query.body {
            // look at all the terms in the atom
            for term in &atom.terms {
                // if the term is the variable we are looking at
                if term == &Term::Variable(var.to_string()) {
                    // find the index of the term in the atom
                    let mut index = 0;
                    for i in 0..atom.terms.len() {
                        if &atom.terms[i] == term {
                            index = i;
                            break;
                        }
                    }
                    // add the attribute at that index from the right relation to the projection attributes
                    for relation in &database.relations {
                        if relation.name == atom.relation_name {
                            projectionattributes.push(relation.attributes[index].to_owned());
                        }
                    }

                }
            }
        }

    // perform the joins
    for join in &joins {
        naturaljoin(join, database, projectionattributes.clone());
    }

}}