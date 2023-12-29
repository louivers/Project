use crate::algorithms::gyo::gyo;
use crate::algorithms::yannakakis::yannakakis;
use crate::models::query::{Query, Atom, Term, ConstantTypes, DataBase, Relation};
use crate::util::loading::read_data;
use serde::Serialize;
use csv::Writer;
use std::error::Error;
use std::fs::File;

// A struct to represent a row in the CSV file.
#[derive(Debug, Serialize)]
struct QueryResult {
    query_id: usize,
    is_acyclic: i32,
    bool_answer: Option<i32>,
    attr_x_answer: Option<Vec<i32>>,
    attr_y_answer: Option<Vec<i32>>,
    attr_z_answer: Option<Vec<i32>>,
    attr_w_answer: Option<Vec<i32>>,
}



pub fn test_queries() -> Result<(), Box<dyn Error>> {
    let beers = read_data(String::from("data/beers.csv"));
    let breweries = read_data(String::from("data/breweries.csv"));
    let categories = read_data(String::from("data/categories.csv"));
    let locations = read_data(String::from("data/locations.csv"));
    let styles = read_data(String::from("data/styles.csv"));
    let mut db = match (beers, breweries, categories, locations, styles) {
        (Ok(beers), Ok(breweries), Ok(categories), Ok(locations), Ok(styles)) => DataBase::from_record_batches(
            vec![
                beers, 
                breweries, 
                categories, 
                locations,
                styles], 
            vec![String::from("beers"),
                String::from("breweries"),
                String::from("categories"),
                String::from("locations"),
                String::from("styles")]),
        _ => panic!("Error loading database."),
    };
    let test1 = Query {
        head: vec![],
        body: vec![
            Atom {
                relation_name: String::from("beers"),
                terms: vec![
                    Term::Variable(String::from("u1")),
                    Term::Variable(String::from("x")),
                    Term::Variable(String::from("u2")),
                    Term::Constant(ConstantTypes::Float(0.07)),
                    Term::Variable(String::from("u3")),
                    Term::Variable(String::from("u4")),
                    Term::Variable(String::from("y")),
                    Term::Variable(String::from("u5")),
                ],
            },
            Atom {
                relation_name: String::from("styles"),
                terms: vec![
                    Term::Variable(String::from("u6")),
                    Term::Variable(String::from("z")),
                    Term::Variable(String::from("y")),
                ],
            },
            Atom {
                relation_name: String::from("categories"),
                terms: vec![
                    Term::Variable(String::from("z")),
                    Term::Variable(String::from("u7")),
                ],
            },
            Atom {
                relation_name: String::from("locations"),
                terms: vec![
                    Term::Variable(String::from("u8")),
                    Term::Variable(String::from("x")),
                    Term::Variable(String::from("u9")),
                    Term::Variable(String::from("u10")),
                    Term::Variable(String::from("u11")),
                ],
            },
            Atom {
                relation_name: String::from("breweries"),
                terms: vec![
                    Term::Variable(String::from("x")),
                    Term::Variable(String::from("u12")),
                    Term::Variable(String::from("u13")),
                    Term::Variable(String::from("u14")),
                    Term::Variable(String::from("u15")),
                    Term::Variable(String::from("u16")),
                    Term::Variable(String::from("u17")),
                    Term::Variable(String::from("u18")),
                    Term::Variable(String::from("u13")),
                    Term::Variable(String::from("u14")),
                    Term::Variable(String::from("u15")),
                ],
            },
        ],
    };
    let test2 = Query {
        head: vec![
            String::from("x"),
            String::from("y"),
            String::from("z"),
        ],
        body: vec![
            Atom {
                relation_name: String::from("breweries"),
                terms: vec![
                    Term::Variable(String::from("w")),
                    Term::Variable(String::from("x")),
                    Term::Constant(ConstantTypes::Utf8(String::from("Westmalle"))),
                    Term::Variable(String::from("u1")),
                    Term::Variable(String::from("u2")),
                    Term::Variable(String::from("u3")),
                    Term::Variable(String::from("u4")),
                    Term::Variable(String::from("u5")),
                    Term::Variable(String::from("u6")),
                    Term::Variable(String::from("u7")),
                    Term::Variable(String::from("u8")),
                ],
            },
            Atom {
                relation_name: String::from("locations"),
                terms: vec![
                    Term::Variable(String::from("u9")),
                    Term::Variable(String::from("w")),
                    Term::Variable(String::from("y")),
                    Term::Variable(String::from("z")),
                    Term::Variable(String::from("u10")),
                ],
            },
        ],
    };
    let test3 = Query {
        head: vec![
            String::from("x"),
            String::from("y"),
            String::from("z"),
        ],
        body: vec![
            Atom {
                relation_name: String::from("beers"),
                terms: vec![
                    Term::Variable(String::from("u1")),
                    Term::Variable(String::from("u2")),
                    Term::Variable(String::from("z")),
                    Term::Variable(String::from("u3")),
                    Term::Variable(String::from("u4")),
                    Term::Variable(String::from("u5")),
                    Term::Variable(String::from("x")),
                    Term::Variable(String::from("u6")),
                ],
            },
            Atom {
                relation_name: String::from("styles"),
                terms: vec![
                    Term::Variable(String::from("u7")),
                    Term::Variable(String::from("y")),
                    Term::Variable(String::from("x")),
                ],
            },
            Atom {
                relation_name: String::from("categories"),
                terms: vec![
                    Term::Variable(String::from("y")),
                    Term::Variable(String::from("z")),
                ],
            },
        ],
    };
    let test4 = Query {
        head: vec![
            String::from("x"),
            String::from("y"),
            String::from("z"),
            String::from("w"),
        ],
        body: vec![
            Atom {
                relation_name: String::from("beers"),
                terms: vec![
                    Term::Variable(String::from("u1")),
                    Term::Variable(String::from("v")),
                    Term::Variable(String::from("x")),
                    Term::Constant(ConstantTypes::Float(0.05)),
                    Term::Constant(ConstantTypes::Int(18)),
                    Term::Variable(String::from("u2")),
                    Term::Constant(ConstantTypes::Utf8(String::from("Vienna Lager"))),
                    Term::Variable(String::from("u3")),
                ],
            },
            Atom {
                relation_name: String::from("locations"),
                terms: vec![
                    Term::Variable(String::from("u4")),
                    Term::Variable(String::from("v")),
                    Term::Variable(String::from("y")),
                    Term::Variable(String::from("z")),
                    Term::Variable(String::from("w")),
                ],
            },
        ],
    };
    let test5 = Query {
        head: vec![
            String::from("x"),
            String::from("y"),
            String::from("z"),
            String::from("w"),
        ],
        body: vec![
            Atom {
                relation_name: String::from("beers"),
                terms: vec![
                    Term::Variable(String::from("u1")),
                    Term::Variable(String::from("x")),
                    Term::Variable(String::from("u2")),
                    Term::Constant(ConstantTypes::Float(0.06)),
                    Term::Variable(String::from("u3")),
                    Term::Variable(String::from("u4")),
                    Term::Variable(String::from("y")),
                    Term::Variable(String::from("u5")),
                ],
            },
            Atom {
                relation_name: String::from("styles"),
                terms: vec![
                    Term::Variable(String::from("u6")),
                    Term::Variable(String::from("z")),
                    Term::Variable(String::from("y")),
                ],
            },
            Atom {
                relation_name: String::from("categories"),
                terms: vec![
                    Term::Variable(String::from("z")),
                    Term::Variable(String::from("w")),
                ],
            },
            Atom {
                relation_name: String::from("locations"),
                terms: vec![
                    Term::Variable(String::from("u8")),
                    Term::Variable(String::from("x")),
                    Term::Variable(String::from("u9")),
                    Term::Variable(String::from("u10")),
                    Term::Variable(String::from("u11")),
                ],
            },
            Atom {
                relation_name: String::from("breweries"),
                terms: vec![
                    Term::Variable(String::from("x")),
                    Term::Variable(String::from("u12")),
                    Term::Variable(String::from("u13")),
                    Term::Variable(String::from("u14")),
                    Term::Variable(String::from("u15")),
                    Term::Variable(String::from("u16")),
                    Term::Variable(String::from("u17")),
                    Term::Variable(String::from("u18")),
                    Term::Variable(String::from("u13")),
                    Term::Variable(String::from("u14")),
                    Term::Variable(String::from("u15")),
                ],
            },
        ],
    };
    let queries = vec![test1, test2, test3, test4, test5];
    let file = File::create("output.csv")?;
    let mut writer = Writer::from_writer(file);
    for (i, query) in queries.iter().enumerate() {
        // Check if the query is acyclic and evaluate it.
        let gyo_res = gyo(query);
        let mut evaluation_res: Option<Relation> = None;
        if gyo_res {
            evaluation_res = Some(yannakakis(query.clone(), &mut db));
        }
        let is_acyclic = match gyo_res {
            true => 1,
            false => 0,
        };
        // Create a new QueryResult to represent the row.
        let result = QueryResult {
            query_id: i + 1,
            is_acyclic,
            bool_answer: None,
            attr_x_answer: None,
            attr_y_answer: None,
            attr_z_answer: None,
            attr_w_answer: None, 
        };

        // Write the QueryResult to the CSV file.
        writer.serialize(result)?;
    }
    writer.flush()?;

    Ok(())
}