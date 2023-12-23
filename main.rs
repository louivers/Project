mod algorithms;
mod models;

use algorithms::gyo;
use models::query;

fn main() {
    // make vectors of variables which is a type of Term
    let r_atoms = vec![
        query::Term::Variable("x"),
        query::Term::Variable("y"),
        query::Term::Variable("z"),
    ];
    let s_atoms = vec![
        query::Term::Variable("a"),
        query::Term::Variable("b"),
        query::Term::Variable("x"),
    ];
    let p_atoms = vec![query::Term::Variable("b")];
    let my_body = vec![
        query::Atom {
            relation_name: "R",
            terms: r_atoms,
        },
        query::Atom {
            relation_name: "S",
            terms: s_atoms,
        },
        query::Atom {
            relation_name: "P",
            terms: p_atoms,
        },
    ];
    let my_query = query::Query {
        head: vec!["x"],
        body: my_body,
    };
    println!("{:#?}", gyo::gyo(&my_query));
}
