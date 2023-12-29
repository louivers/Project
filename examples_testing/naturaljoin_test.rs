use crate::{models::query::{Relation, ConstantTypes, self, NaturalJoin}, algorithms::gyo::naturaljoin};

#[allow(dead_code)]
fn to_constant_types_vec(ints: Vec<i32>) -> Vec<ConstantTypes> {
    ints.into_iter().map(ConstantTypes::Int).collect()
}

#[allow(dead_code)]
#[allow(unused_variables)]
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
    let r5 = Relation {
        name: "R5".to_string(),
        arity: 3,
        attributes: vec!["A".to_string(), "B".to_string(), "C".to_string()],
        tuples: vec![vec![1, 3,4], vec![2,3,4]]
            .into_iter().map(to_constant_types_vec).collect(),
    };
    let r6 = Relation {
        name: "R6".to_string(),
        arity: 3,
        attributes: vec!["B".to_string(), "C".to_string(), "D".to_string()],
        tuples: vec![vec![3, 4, 5], vec![3,4,6]]
            .into_iter().map(to_constant_types_vec).collect(),
    };
    let r7 = Relation {
        name: "R7".to_string(),
        arity: 2,
        attributes: vec!["B".to_string(), "F".to_string()],
        tuples: vec![vec![3, 8], vec![3,9]]
            .into_iter().map(to_constant_types_vec).collect(),
    };

    let mut db = crate::models::query::DataBase {
        relations: vec![r5, r6, r7],
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
    let r5_atoms = vec![
        query::Term::Variable(String::from("A")),
        query::Term::Variable(String::from("B")),
        query::Term::Variable(String::from("C")),
    ];
    let r6_atoms = vec![
        query::Term::Variable(String::from("B")),
        query::Term::Variable(String::from("C")),
        query::Term::Variable(String::from("D")),
    ];
    let r7_atoms = vec![
        query::Term::Variable(String::from("B")),
        query::Term::Variable(String::from("F")),
    ];
    let my_body = vec![
        query::Atom {
            relation_name: String::from("R5"),
            terms: r5_atoms.clone(),
        },
        query::Atom {
            relation_name: String::from("R6"),
            terms: r6_atoms.clone(),
        },
    ];
    let my_query = query::Query {
        head: vec![String::from("A"), String::from("C")],
        body: my_body,
    };

    // make the natural join
    let join = NaturalJoin {
        left: query::Atom {
                relation_name: String::from("R5"),
                terms: r4_atoms.clone(),
        },
        right: query::Atom {
                relation_name: String::from("R6"),
                terms: r2_atoms.clone(),
        },
    };

    // print the database
    println!("Database before natural join:");
    println!("{}", join);
    println!("{}", db);
    
    // try natural join
    naturaljoin(join, &mut db, my_query.body.clone());

    // print the database
    println!("Database after natural join:");
    println!("{}", db);

    // make the natural join
    let join2 = NaturalJoin {
        left: query::Atom {
                relation_name: String::from("R7"),
                terms: r4_atoms,
        },
        right: query::Atom {
                relation_name: String::from("R6"),
                terms: r2_atoms,
        },
    };

    // print the database
    println!("Database before natural join:");
    println!("{}", join2);
    println!("{}", db);
    
    // try natural join
    naturaljoin(join2, &mut db, my_query.body);

    // print the database
    println!("Database after natural join:");
    println!("{}", db)
    
}