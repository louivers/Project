use std::fmt;

use arrow::{record_batch::RecordBatch, array::ArrayRef, datatypes::DataType};

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
    Int(i64),
    Null,
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

pub struct NaturalJoin {
    pub left: Atom,
    pub right: Atom,
}

impl fmt::Display for SemiJoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} ⋉ {})", self.left.relation_name, self.right.relation_name)
    }
}

impl fmt::Display for NaturalJoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} ⋈ {})", self.left.relation_name, self.right.relation_name)
    }
}

pub struct DataBase {
    pub relations: Vec<Relation>,
}

impl fmt::Display for DataBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for relation in &self.relations {
            write!(f, "{}\n", relation.name)?;
            // write attributes
            write!(f, "\t")?;
            for attribute in &relation.attributes {
                write!(f, "{} ", attribute)?;
            }
            write!(f, "\n")?;
            for tuple in &relation.tuples {
                write!(f, "\t")?;
                for term in tuple {
                    write!(f, "{:?} ", term)?;
                }
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

pub struct Relation{
    pub name: String,
    pub arity: usize,
    pub attributes: Vec<String>,
    pub tuples: Vec<Vec<ConstantTypes>>,
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.name)?;
        // write attributes
        write!(f, "\t")?;
        for attribute in &self.attributes {
            write!(f, "{} ", attribute)?;
        }
        write!(f, "\n")?;
        for tuple in &self.tuples {
            write!(f, "\t")?;
            for term in tuple {
                write!(f, "{:?} ", term)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
impl Relation {
    #[allow(dead_code)]
    pub fn from_record_batch(batch: RecordBatch, name: String) -> Self {
        let schema = batch.schema();
        let fields = schema.fields();
        let name = name;
        let arity = fields.len();
        let attributes: Vec<String> = fields.iter().map(|f| f.name().clone()).collect();
        let columns: Vec<&ArrayRef> = batch.columns().iter().collect();
        let column_data_types: Vec<DataType> = columns.iter().map(|c| c.data_type().clone()).collect();
        let num_rows = batch.num_rows();
        let mut tuple: Vec<ConstantTypes> = Vec::new();
        for row_idx in 0..num_rows {            
            for col_idx in 0..arity {
                let col = columns[col_idx];
                let data_type = &column_data_types[col_idx];
                match data_type {
                    DataType::Utf8 => {
                        let string_column = col.as_any().downcast_ref::<arrow::array::StringArray>().unwrap();
                        let value = string_column.value(row_idx);
                        tuple.push(ConstantTypes::Utf8(value.to_string()));
                    },
                    DataType::Float64 => {
                        let float_column = col.as_any().downcast_ref::<arrow::array::Float64Array>().unwrap();
                        let value = float_column.value(row_idx);
                        tuple.push(ConstantTypes::Float(value));
                    },
                    DataType::Int64 => {
                        let int_column = col.as_any().downcast_ref::<arrow::array::Int64Array>().unwrap();
                        let value = int_column.value(row_idx);
                        tuple.push(ConstantTypes::Int(value));
                    },
                    DataType::Null => {
                        tuple.push(ConstantTypes::Null);
                    },
                    _ => panic!("Unsupported data type"),
                }
            }
        }
        return Relation{
            name: name.clone(),
            arity: arity,
            attributes: attributes.clone(),
            tuples: vec![tuple],
        };
    }
}

impl DataBase {
    #[allow(dead_code)]
    pub fn from_record_batches(batch: Vec<RecordBatch>, names: Vec<String>) -> DataBase {
        let mut tuples: Vec<Relation> = Vec::new();
        for i in 0..batch.len() {
            let relation = Relation::from_record_batch(batch[i].clone(), names[i].clone());
            tuples.push(relation);
        }
        return DataBase{
            relations: tuples,
        };
    }
}