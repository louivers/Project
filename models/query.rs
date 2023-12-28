use std::fmt;

#[derive(Debug)]
pub struct Query {
    pub head: Vec<String>,
    pub body: Vec<Atom>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Atom {
    pub relation_name: String,
    pub terms: Vec<Term>,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum ConstantTypes {
    Utf8(String),
    Float(f64),
    Int(i32),
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Term {
    Variable(String),
    Constant(ConstantTypes),
}

impl Atom {
    pub fn same_vars(&self, other: &Atom) -> bool {
        let mut vars1 = Vec::new();
        let mut vars2 = Vec::new();
        for term in &self.terms {
            if let Term::Variable(var) = term {
                vars1.push(var.to_owned());
            }
        }
        for term in &other.terms {
            if let Term::Variable(var) = term {
                vars2.push(var.to_owned());
            }
        }
        return vars1 == vars2;
    }
}

#[allow(dead_code)]
pub struct SemiJoin {
    pub left: Atom,
    pub right: Atom,
}

// make a good print for SemiJoin
impl fmt::Display for SemiJoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} ⋈ {})", self.left.relation_name, self.right.relation_name)
    }
}

pub struct DataBase {
    pub relations: Vec<Relation>,
}

pub struct Relation{
    pub name: String,
    pub arity: usize,
    pub attributes: Vec<String>,
    pub tuples: Vec<Vec<ConstantTypes>>,

}