mod models;
mod algorithms;

use models::query;
use algorithms::gyo;

fn main() {
    let my_terms = vec![
        query::Term::Variable("x"),
        query::Term::Constant(query::ConstantTypes::Utf8(String::from("test_utf8_const"))),
    ];
    let my_body = vec![
        query::Atom {
            relation_name: String::from("R"),
            terms: my_terms,
        }
    ];
    let my_query = query::Query {
        head: vec!["abc"],
        body: my_body
    };
    println!("{:#?}", my_query);
    gyo::find_ear(&my_query);
}