use petgraph::{Graph, Directed};
use crate::models::query::{Atom, Query, Term};
// use crate::models::join_tree::JoinTree;

// Returns true if the given query is acyclic. False otherwise.
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

#[allow(dead_code)]
pub fn generate_join_tree(atoms: &Vec<Atom>) -> Option<Graph<Atom, u8, Directed>> {
    let first_ear = find_ear(&atoms);
    println!("first ear: {:?}", first_ear);
    if first_ear.0.is_none() {
        return None;
    }
    let mut my_atoms = remove_atom(&atoms, first_ear.0.unwrap());
    let mut join_tree = Graph::<Atom, u8, Directed>::new();

    let idx_ear = join_tree.add_node(first_ear.0.unwrap().to_owned());
    let idx_witness = join_tree.add_node(first_ear.1.unwrap().to_owned());
    join_tree.add_edge(idx_witness, idx_ear, 0);
    while my_atoms.len() != 0 {
        let ear = find_ear(&my_atoms);
        println!("ear: {:?}", ear);
        if ear.0.is_none() {
            return None;
        } else if ear.1.is_none() {
            return Some(join_tree);
        } else {
            let new_atoms = remove_atom(&my_atoms, ear.1.unwrap());
            let mut idx_ear = None;
            let mut idx_witness = None;
            for node in join_tree.node_indices() {
                if join_tree[node].same_vars(&ear.0.unwrap()) {
                    idx_ear = Some(node);
                }
                if join_tree[node].same_vars(&ear.1.unwrap()) {
                    idx_witness = Some(node);
                }
            }
            if idx_ear == None {
                idx_ear = Some(join_tree.add_node(ear.0.unwrap().to_owned()));
            }
            if idx_witness == None {
                idx_witness = Some(join_tree.add_node(ear.1.unwrap().to_owned()));
            }
            join_tree.add_edge(idx_witness.unwrap(), idx_ear.unwrap(), 0);
            my_atoms = new_atoms;
        }
    } 
    return Some(join_tree);
}

fn remove_atom(atoms: &Vec<Atom>, atom: &Atom) -> Vec<Atom> {
    let new_atoms: Vec<Atom> = atoms.to_vec().into_iter().filter(|a| a != atom).collect();
    return new_atoms.to_vec();
}

pub fn find_ear(atoms: &Vec<Atom>) -> (Option<&Atom>, Option<&Atom>) {
    if atoms.len() == 1 {
        return (Some(&atoms[0]), None);
    }
    for atom in atoms {
        let witness = find_witness(atoms, atom);
        if witness == None {
            continue;
        } else {
            return (Some(atom), witness);
        }
    }
    return (None, None);
}

#[allow(dead_code)]
fn find_witness<'a>(atoms: &'a Vec<Atom>, a: &Atom) -> Option<&'a Atom> {
    let mut potential_witness: Option<&Atom> = None;
    for term in &a.terms {
        let mutual_atoms = find_other_atoms_with_term(atoms, a, &term);
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
pub fn find_other_atoms_with_term<'a>(atoms: &'a Vec<Atom>, from_atom: &Atom, term: &Term) -> Vec<&'a Atom> {
    let mut result = Vec::new();
    for atom in atoms {
        if atom == from_atom {
            continue
        } else {
            if atom.terms.contains(&term) {
                result.push(atom);
            }
        }
    }
    return result;
}