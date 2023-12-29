use crate::{models::query::{Relation, ConstantTypes, self}, algorithms::gyo::naturaljoin};

#[allow(dead_code)]
fn to_constant_types_vec(ints: Vec<i32>) -> Vec<ConstantTypes> {
    ints.into_iter().map(ConstantTypes::Int).collect()
}

#[allow(dead_code)]
pub fn naturaljoin_test() {
    let r1 = Relation {
        name: "R1".to_string(),
        arity: 2,
        attributes: vec!["A1".to_string(), "A2".to_string()],
        tuples: vec![vec![1, 20], vec![1, 10], vec![4, 60]]
            .into_iter().map(to_constant_types_vec).collect(),
    };
    let r2 = Relation {
        name: "R2".to_string(),
        arity: 3,
        attributes: vec!["A1".to_string(), "A2".to_string(), "A3".to_string()],
        tuples: vec![vec![1, 10, 100], vec![1, 20, 100], vec![3, 10, 300]
            , vec![1, 40, 300], vec![2, 30, 200]]
            .into_iter().map(to_constant_types_vec).collect(),
    };
    let r3 = Relation {
        name: "R3".to_string(),
        arity: 1,
        attributes: vec!["A2".to_string()],
        tuples: vec![vec![10], vec![20], vec![30]]
            .into_iter().map(to_constant_types_vec).collect(),
    };
    let r4 = Relation {
        name: "R4".to_string(),
        arity: 3,
        attributes: vec!["A1".to_string(), "A2".to_string(), "A4".to_string()],
        tuples: vec![vec![1, 10, 1000], vec![1, 20, 1000], vec![1,20, 2000], 
                     vec![2, 20,2000]]
            .into_iter().map(to_constant_types_vec).collect(),
    };

    let mut db = crate::models::query::DataBase {
        relations: vec![r1, r2, r3, r4],
    };

    // make a query
    let r1_atoms = vec![
        query::Term::Variable(String::from("A1")),
        query::Term::Variable(String::from("A2")),
    ];
    let r2_atoms = vec![
        query::Term::Variable(String::from("A1")),
        query::Term::Variable(String::from("A2")),
        query::Term::Variable(String::from("A3")),
    ];
    let r3_atoms = vec![query::Term::Variable(String::from("A2"))];
    let r4_atoms = vec![
        query::Term::Variable(String::from("A1")),
        query::Term::Variable(String::from("A2")),
        query::Term::Variable(String::from("A4")),
    ];
    let my_body = vec![
        query::Atom {
            relation_name: String::from("R1"),
            terms: r1_atoms,
        },
        query::Atom {
            relation_name: String::from("R2"),
            terms: r2_atoms,
        },
        query::Atom {
            relation_name: String::from("R3"),
            terms: r3_atoms,
        },
        query::Atom {
            relation_name: String::from("R4"),
            terms: r4_atoms,
        },
    ];
    let my_query = query::Query {
        head: vec![String::from("A1"), String::from("A2")],
        body: my_body,
    };

    //naturaljoin();

    
}