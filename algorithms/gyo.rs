use crate::models::query::{Atom, Query, Term};

pub fn find_ear<'a>(q: &'a Query) -> Option<Atom<'a>> {
    let atoms = &q.body;
    for atom in atoms {
        println!("{:#?}", atom);
    }
    return None;
}

#[allow(dead_code)]
fn is_ear(a: &Atom) -> bool {
    return true;
}

#[allow(dead_code)]
fn find_witness<'a>(q: &'a Query, a: &'a Atom) -> Option<Atom<'a>> {
    let atoms = &q.body;
    for atom in atoms {
        if atom == a {
            continue;
        } else {

        }
    }
    return None;
}

#[allow(dead_code)]
fn find_other_atom_with_term<'a>(q: &'a Query, from_atom: &'a Atom, term: &'a Term) -> Vec<&'a Atom<'a>> {
    let mut result = Vec::new();
    for atom in &q.body {
        if atom == from_atom {
            continue
        } else {
            if atom.terms.contains(term) {
                result.push(atom);
            }
        }
    }
    return result;
}