use arrow::datatypes::DataType;

pub struct Query {
    pub head: String,
    pub body: Vec<Atom>,
}

#[derive(Debug)]
pub struct Atom {
    pub relation_name: String,
    pub terms: Vec<Term>,
}

#[derive(Debug)]
pub enum Term {
    Variable(String),
    Constant(DataType),
}

pub fn display_query(q: Query) {
    let head = q.head;
    let body = q.body;
    println!("Answer({head}): {:?}", body);
}