use crate::models::query::{Query, DataBase, NaturalJoin, Term, ConstantTypes, Relation};
use crate::algorithms::helper::{find_root, post_order_apply};
use crate::algorithms::join_tree::generate_join_tree;
use crate::algorithms::full_reducer::globally_consistent_database;
use crate::algorithms::joins::naturaljoin;

use super::gyo;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn yannakakis(query:Query, database: &mut DataBase)  -> Relation{
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
    }

    // perform the joins
    for join in &joins {
        naturaljoin(join, database, projectionattributes.clone());
    }

    // select the relation that was last joined
    let mut last_relation = None;
    for relation in &database.relations {
        if relation.name == joins[joins.len()-1].right.relation_name {
            last_relation = Some(relation);
        }
    }

    // project the attributes
    //if the last relation is empty, return an empty relation
    if last_relation.is_none() {
        return Relation {
            name: String::from("ANSWER"),
            attributes: projectionattributes.clone(),
            // return true tuple
            tuples: vec![vec![ConstantTypes::Utf8("FALSE".to_string())]],
            arity: projectionattributes.len(),
        };
    }
    let mut new_tuples: Vec<Vec<ConstantTypes>> = Vec::new();
    for tuple in &last_relation.unwrap().tuples {
        let mut new_tuple: Vec<ConstantTypes> = Vec::new();
        for attribute in &projectionattributes {
            for i in 0..last_relation.unwrap().attributes.len() {
                if last_relation.unwrap().attributes[i] == *attribute {
                    new_tuple.push(tuple[i].clone());
                }
            }
        }
        // if the new tuple is not already in the new tuples, add it
        if !new_tuples.contains(&new_tuple) {
            new_tuples.push(new_tuple);
        }
    }

    // make the new relation
    let new_relation = 
        Relation {
            name: String::from("ANSWER"),
            attributes: projectionattributes.clone(),
            tuples: new_tuples.clone(),
            arity: projectionattributes.len(),
    };

    // return the new relation
    return new_relation;
}