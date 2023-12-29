use crate::algorithms::join_tree::generate_join_tree;
use crate::models::query;

#[allow(dead_code)]
pub fn join_tree_test() {
    // make vectors of variables which is a type of Term
    let r_atoms = vec![
        query::Term::Variable(String::from("x")),
        query::Term::Variable(String::from("y")),
        query::Term::Variable(String::from("z")),
    ];
    let s_atoms = vec![
        query::Term::Variable(String::from("a")),
        query::Term::Variable(String::from("b")),
        query::Term::Variable(String::from("x")),
    ];
    let p_atoms = vec![query::Term::Variable(String::from("b"))];
    let my_body = vec![
        query::Atom {
            relation_name: String::from("R"),
            terms: r_atoms,
        },
        query::Atom {
            relation_name: String::from("S"),
            terms: s_atoms,
        },
        query::Atom {
            relation_name: String::from("P"),
            terms: p_atoms,
        },
    ];
    let my_query = query::Query {
        head: vec![String::from("x")],
        body: my_body,
    };
    // println!("{:#?}", gyo::gyo(&my_query));
    let join_tree = generate_join_tree(&my_query.body);
    println!("THIS IS THE JOIN TREE");
    println!("{:#?}", join_tree);
}