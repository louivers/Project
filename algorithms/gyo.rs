use crate::models::query::{Atom, Query, Term};

// returns true if the given query is acyclic. False otherwise.
#[allow(dead_code)]
pub fn gyo(q: &Query) -> bool {
    let mut atoms = q.body.clone();
    while atoms.len() != 1 {
        let ear = find_ear(&atoms);
        if ear.0.is_none() {
            return false;
        } else {
            atoms = remove_atom(&atoms, ear.0.unwrap());
        }
    }
    return true;
}

// same as gyo, but also returns all ears and their witnesses
pub fn gyo_return_steps(q: &Query) -> (bool, Vec<(Atom, Atom)>) {
    let mut atoms = q.body.clone();
    let mut steps = Vec::new();
    while atoms.len() != 1 {
        let atoms_clone = atoms.clone();
        let ear = find_ear(&atoms_clone);
        if ear.0.is_none() {
            return (false, steps);
        } else {
            atoms = remove_atom(&atoms, ear.0.unwrap());
            steps.push((ear.0.unwrap().to_owned(), ear.1.unwrap().to_owned()));
        }
    }
    return (true, steps);
}

// removes an atom from a vector of atoms
fn remove_atom(atoms: &Vec<Atom>, atom: &Atom) -> Vec<Atom> {
    let new_atoms: Vec<Atom> = atoms.to_vec().into_iter().filter(|a| a != atom).collect();
    return new_atoms.to_vec();
}

// returns true if the given term is exclusive to the given atom
pub fn exclusive_term(atoms: &Vec<Atom>, a: &Atom, term: &Term) -> bool {
    let mut exclusive = true;
    for atom in atoms {
        if atom == a {
            continue;
        }
        if atom.terms.contains(&term) {
            exclusive = false;
            break;
        }
    }
    return exclusive;
}

// returns the ear and its witness if there is one
pub fn find_ear(atoms: &Vec<Atom>) -> (Option<&Atom>, Option<&Atom>) {
    // if there is only one atom, it is an ear
    if atoms.len() == 1 {
        return (Some(&atoms[0]), None);
    }
    // an atom is an ear if
    for atom in atoms {    
        // 1 : all vertices of the hyperedge are exclusive to that hyperedge
        let mut exclusive = true;
        for term in &atom.terms {
            if !exclusive_term(atoms, atom, term) {
                exclusive = false;
                break;
            }
        }
        if exclusive {
            return (Some(atom), None);
        }
        // 2 : there is a witness for that atom
        let witness = find_witness(atoms, atom);
        if witness == None {
            continue;
        } else {
            return (Some(atom), witness);
        }
    }
    return (None, None);
}

// find a witness for the given atom
pub fn find_witness<'a>(atoms: &'a Vec<Atom>, a: &Atom) -> Option<&'a Atom> {
    for potential_witness in atoms {
        let mut is_witness = true;
        if potential_witness == a {
            continue;
        } else {
            for term in &a.terms {
                // every term of the atom is either exclusive to the atom 
                if !exclusive_term(atoms, a, term) && !potential_witness.terms.contains(&term){
                    is_witness = false;
                    break;
                }
                // or it is also in the witness
            }
        }
        if is_witness {
           return Some(potential_witness);
        }
    }
    return None;
}