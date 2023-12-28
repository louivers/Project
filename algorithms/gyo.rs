use crate::models::query::{Atom, Query, SemiJoin, Term};
use petgraph::{Directed, Graph};
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
pub fn find_other_atoms_with_term<'a>(
    atoms: &'a Vec<Atom>,
    from_atom: &Atom,
    term: &Term,
) -> Vec<&'a Atom> {
    let mut result = Vec::new();
    for atom in atoms {
        if atom == from_atom {
            continue;
        } else {
            if atom.terms.contains(&term) {
                result.push(atom);
            }
        }
    }
    return result;
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn build_full_reducer_from_tree(join_tree: &Graph<Atom, u8, Directed>) -> Vec<SemiJoin> {
    // the full reducer is a vector of SemiJoins
    let mut reducer = Vec::new();
    // find a root in the join tree
    let mut root = find_root(join_tree).unwrap();
    // bottom up traversal of the join tree to build first semijoins of the reducer
    post_order_apply(join_tree, petgraph::graph::NodeIndex::new(root), &mut |join_tree, node| {
        // take the node from the join tree and find its parent
        let parent = join_tree
            .neighbors_directed(node, petgraph::Direction::Incoming)
            .next();
        // if the node has no parent, it is the root of the join tree 
        // nothing needs to be added to the reducer
        if parent == None {
            return;
        } else {
            // if the node has a parent, add a semijoin to the reducer
            // the semijoin has the node as its right child and the parent as its left child
            let semijoin = SemiJoin{
                right: join_tree[node].to_owned(),
                left: join_tree[parent.unwrap()].to_owned(),
            };
            reducer.push(semijoin);
        }
    });
    
    // top down traversal of the join tree to build the rest of the reducer
    pre_order_apply(join_tree, petgraph::graph::NodeIndex::new(root), &mut |join_tree, node| {
        // take the node from the join tree and find its children
        let children = join_tree
            .neighbors_directed(node, petgraph::Direction::Outgoing)
            .collect::<Vec<_>>();
        // if the node has no children, it is a leaf of the join tree
        // nothing needs to be added to the reducer
        if children.len() == 0 {
            return;
        } else {
            // if the node has children, add a semijoin to the reducer for each direct child
            // the semijoin has the node as its right and the first child as its left
            for child in children.iter() {
                let semijoin = SemiJoin{
                    right: join_tree[node].to_owned(),
                    left: join_tree[*child].to_owned(),
                };
                reducer.push(semijoin);
            }
            
        }
    });
    return reducer;
}

// Returns the index of the root of the given join tree.
pub fn find_root(join_tree: &Graph<Atom, u8, Directed>) -> Option<usize> {
    for node in join_tree.node_indices() {
        if join_tree.neighbors_directed(node, petgraph::Direction::Incoming).count() == 0 {
            return Some(node.index());
        }
    }
    return None;
}

pub fn post_order_apply<F>(
    join_tree: &Graph<Atom, u8, Directed>,
    root: petgraph::graph::NodeIndex,
    funct: &mut F)
    where F: FnMut(&Graph<Atom, u8, Directed>, petgraph::graph::NodeIndex) {
    let mut stack = Vec::new();
    let mut visited = vec![false; join_tree.node_count()];
    stack.push(root);
    while !stack.is_empty() {
        let node = *stack.last().unwrap();
        if !visited[node.index()] {
            for neighbor in join_tree.neighbors_directed(node, petgraph::Direction::Outgoing) {
                stack.push(neighbor);
            }
            visited[node.index()] = true;
        } else {
            stack.pop();
            funct(join_tree, node);
        }
    }
}

pub fn pre_order_apply<F>(
    join_tree: &Graph<Atom, u8, Directed>,
    root: petgraph::graph::NodeIndex,
    funct: &mut F)
    where F: FnMut(&Graph<Atom, u8, Directed>, petgraph::graph::NodeIndex) {
    let mut stack = Vec::new();
    stack.push(root);
    while stack.len() != 0 {
        let node = stack.pop().unwrap();
        funct(join_tree, node);
        for neighbor in join_tree.neighbors_directed(node, petgraph::Direction::Outgoing) {
            stack.push(neighbor);
        }

    }}
