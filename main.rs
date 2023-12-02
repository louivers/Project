mod models;

pub use models::query;

fn main() {
    let my_terms = vec![
        query::Term::Variable(String::from("test_constant")),
    ];
    let my_body = vec![
        query::Atom {
            relation_name: String::from("test_relation_name"),
            terms: my_terms,
        }
    ];
    let my_query = query::Query {
        head: String::from("test_head"),
        body: my_body
    };
    query::display_query(my_query);
}