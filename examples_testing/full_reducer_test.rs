use crate::models::query::{Atom, Term};
use crate::algorithms::full_reducer::build_full_reducer_from_tree;
use crate::algorithms::join_tree::generate_join_tree;

#[allow(dead_code)]
pub fn full_reducer_test() {
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
    
    let full_reducer = build_full_reducer_from_tree(&join_tree);
    //print the full reducer using the Display trait of SemiJoin
    for semijoin in full_reducer {
        println!("{}", semijoin);
    }
}