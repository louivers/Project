#[derive(Debug)]
pub struct Query<'a> {
    pub head: Vec<&'a str>,
    pub body: Vec<Atom<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Atom<'a> {
    pub relation_name: String,
    pub terms: Vec<Term<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum ConstantTypes {
    Utf8(String),
    Float(f64),
    Int(i32),
}

#[derive(Debug, PartialEq)]
pub enum Term<'a> {
    Variable(&'a str),
    Constant(ConstantTypes),
}