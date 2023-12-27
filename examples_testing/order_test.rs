use crate::models::query::{Atom, Term};
use crate::algorithms::gyo::{generate_join_tree, find_root, pre_order_apply, post_order_apply};
use petgraph::graph::NodeIndex;


pub fn order_test() {
    let r_atoms = vec![
        Term::Variable(String::from("x")),
        Term::Variable(String::from("y")),
        Term::Variable(String::from("z")),
    ];
    let s_atoms = vec![
        Term::Variable(String::from("a")),
        Term::Variable(String::from("b")),
        Term::Variable(String::from("x")),
    ];
    let p_atoms = vec![Term::Variable(String::from("b"))];
    let my_body = vec![
        Atom {
            relation_name: String::from("R"),
            terms: r_atoms,
        },
        Atom {
            relation_name: String::from("S"),
            terms: s_atoms,
        },
        Atom {
            relation_name: String::from("P"),
            terms: p_atoms,
        },
    ];
    let my_query = crate::models::query::Query {
        head: vec![String::from("x")],
        body: my_body,
    };
    let join_tree = generate_join_tree(&my_query.body).unwrap();
    let root = find_root(&join_tree).unwrap();
    
    post_order_apply(&join_tree, NodeIndex::new(root), &mut |join_tree, node| {
        println!("post order: {:?}", join_tree[node]);
    });
    pre_order_apply(&join_tree, NodeIndex::new(root), &mut |join_tree, node| {
        println!("pre order: {:?}", join_tree[node]);
    });
}