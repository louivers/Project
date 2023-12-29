use std::vec;

use crate::models::query::{Relation, ConstantTypes, self, NaturalJoin};
use crate::algorithms::joins::naturaljoin;

#[allow(dead_code)]
fn to_constant_types_vec(ints: Vec<i64>) -> Vec<ConstantTypes> {
    ints.into_iter().map(ConstantTypes::Int).collect()
}


// db based on slides from http://infolab.stanford.edu/~ullman/cs345notes/slides01-4.pdf
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn naturaljoin_test() {
    let r1 = Relation {
        name: "R1".to_string(),
        arity: 3,
        attributes: vec!["A".to_string(), "B".to_string(), "C".to_string()],
        tuples: vec![vec![1, 3,4], vec![2,3,4]]
            .into_iter().map(to_constant_types_vec).collect(),
    };
    let r2 = Relation {
        name: "R2".to_string(),
        arity: 3,
        attributes: vec!["B".to_string(), "C".to_string(), "D".to_string()],
        tuples: vec![vec![3, 4, 5], vec![3,4,6]]
            .into_iter().map(to_constant_types_vec).collect(),
    };
    let r3 = Relation {
        name: "R3".to_string(),
        arity: 2,
        attributes: vec!["B".to_string(), "F".to_string()],
        tuples: vec![vec![3, 8], vec![3,9]]
            .into_iter().map(to_constant_types_vec).collect(),
    };
    let r4 = Relation {
        name: "R4".to_string(),
        arity: 2,
        attributes: vec!["C".to_string(), "D".to_string(), "E".to_string()],
        tuples: vec![vec![4, 5,7], vec![4,6,7]]
            .into_iter().map(to_constant_types_vec).collect(),
    };
    let r5 = Relation {
        name: "R5".to_string(),
        arity: 3,
        attributes: vec!["D".to_string(), "E".to_string(), "G".to_string()],
        tuples: vec![vec![5, 7,10], vec![5,7,11], vec![6,7,10]]
            .into_iter().map(to_constant_types_vec).collect(),
    };

    let mut db = crate::models::query::DataBase {
        relations: vec![r1, r2, r3, r4, r5],
    };

    // make a query
    let r1_atoms = vec![
        query::Term::Variable(String::from("A")),
        query::Term::Variable(String::from("B")),
        query::Term::Variable(String::from("C")),
    ];
    let r2_atoms = vec![
        query::Term::Variable(String::from("B")),
        query::Term::Variable(String::from("C")),
        query::Term::Variable(String::from("D")),
    ];
    let r3_atoms = vec![
        query::Term::Variable(String::from("B")),
        query::Term::Variable(String::from("F")),
    ];
    let r4_atoms = vec![
        query::Term::Variable(String::from("C")),
        query::Term::Variable(String::from("D")),
        query::Term::Variable(String::from("E")),
    ];
    let r5_atoms = vec![
        query::Term::Variable(String::from("D")),
        query::Term::Variable(String::from("E")),
        query::Term::Variable(String::from("G")),
    ];

    // make the natural join
    let join = NaturalJoin {
        left: query::Atom {
                relation_name: String::from("R1"),
                terms: r1_atoms.clone(),
        },
        right: query::Atom {
                relation_name: String::from("R2"),
                terms: r2_atoms.clone(),
        },
    };

    // attributes to project
    let mut attributes = vec![];
    attributes.push(String::from("A"));
    attributes.push(String::from("G"));

    // print the database
    println!("Database before natural join:");
    println!("{}", join);
    println!("{}", db);
    
    // try natural join
    naturaljoin(&join, &mut db, attributes.clone());

    // print the database
    println!("Database after natural join:");
    println!("{}", db);

    // make the natural join
    let join2 = NaturalJoin {
        left: query::Atom {
                relation_name: String::from("R3"),
                terms: r3_atoms.clone(),
        },
        right: query::Atom {
                relation_name: String::from("R2"),
                terms: r2_atoms.clone(),
        },
    };

    // print the database
    println!("Database before natural join:");
    println!("{}", join2);
    println!("{}", db);
    
    // try natural join
    naturaljoin(&join2, &mut db, attributes.clone());

    // print the database
    println!("Database after natural join:");
    println!("{}", db);

    // make the natural join
    let join3 = NaturalJoin {
        left: query::Atom {
                relation_name: String::from("R2"),
                terms: r2_atoms.clone(),
        },
        right: query::Atom {
                relation_name: String::from("R4"),
                terms: r4_atoms.clone(),
        },
    };

    // print the database
    println!("Database before natural join:");
    println!("{}", join3);
    println!("{}", db);
    
    // try natural join
    naturaljoin(&join3, &mut db, attributes.clone());

    // print the database
    println!("Database after natural join:");
    println!("{}", db);

    // make the natural join
    let join4 = NaturalJoin {
        left: query::Atom {
                relation_name: String::from("R5"),
                terms: r5_atoms.clone(),
        },
        right: query::Atom {
                relation_name: String::from("R4"),
                terms: r4_atoms.clone(),
        },
    };

    // print the database
    println!("Database before natural join:");
    println!("{}", join4);
    println!("{}", db);
    
    // try natural join
    naturaljoin(&join4, &mut db, attributes.clone());

    // print the database
    println!("Database after natural join:");
    println!("{}", db);
    
}