use crate::models::query::{Atom, Query, Term};

// Returns true if the given query is acyclic. False otherwise.
pub fn gyo(q: &Query) -> bool {
    let mut atoms = q.body.to_vec();
    while ! atoms.len() == 1 {
        let ear = find_ear(&atoms);
        if ear.is_none() {
            return false;
        } else {
            atoms = remove_atom(&atoms, ear.unwrap());
        }
    }
    return true;
}

fn remove_atom<'a>(atoms: &Vec<Atom<'a>>, atom: &Atom) -> Vec<Atom<'a>> {
    let new_atoms: Vec<Atom<'a>> = atoms.to_vec().into_iter().filter(|a| a != atom).collect();
    return new_atoms.to_vec();
}

pub fn find_ear<'a>(atoms: &'a Vec<Atom>) -> Option<&'a Atom<'a>> {
    for atom in atoms {
        let witness = find_witness(atoms, atom);
        if witness == None {
            continue;
        } else {
            return Some(atom);
        }
    }
    return None;
}

#[allow(dead_code)]
fn find_witness<'a>(atoms: &'a Vec<Atom>, a: &'a Atom) -> Option<&'a Atom<'a>> {
    let mut potential_witness: Option<&Atom> = None;
    for term in &a.terms {
        let mutual_atoms = find_other_atoms_with_term(atoms, a, term);
        match mutual_atoms.len() {
            0 => continue,
            1 => {
                if potential_witness == None {
                    potential_witness = Some(mutual_atoms[0]);
                } else if potential_witness == Some(mutual_atoms[0]) {
                    continue;
                } else {
                    potential_witness = None;
                    break;
                }
            }
            _ => break,
        }   
    }
    return potential_witness;
}

#[allow(dead_code)]
pub fn find_other_atoms_with_term<'a>(atoms: &'a Vec<Atom>, from_atom: &Atom, term: &Term) -> Vec<&'a Atom<'a>> {
    let mut result = Vec::new();
    for atom in atoms {
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