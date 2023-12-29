use crate::models::query::{Atom, Query, SemiJoin, Term, DataBase, Relation, ConstantTypes, NaturalJoin, self, };
use petgraph::{Directed, Graph};
// use crate::models::join_tree::JoinTree;

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

// generates a graph representing the join tree if the query is acyclic.
#[allow(dead_code)]
pub fn generate_join_tree(atoms: &Vec<Atom>) -> Option<Graph<Atom, u8, Directed>> {
    // if the query is not acyclic, return None
    let first_ear = find_ear(&atoms);
    println!("first ear: {:?}", first_ear);
    if first_ear.0.is_none() {
        return None;
    }
    // if the query is acyclic, generate the join tree with the first ear as the root
    let mut my_atoms = remove_atom(&atoms, first_ear.0.unwrap());
    let mut join_tree = Graph::<Atom, u8, Directed>::new();

    let idx_ear = join_tree.add_node(first_ear.0.unwrap().to_owned());
    let idx_witness = join_tree.add_node(first_ear.1.unwrap().to_owned());
    join_tree.add_edge(idx_witness, idx_ear, 0);
    let mut last_ear_idx = idx_ear;
    // keep adding nodes to the join tree until there are no atoms left
    while my_atoms.len() != 0 {
        let ear = find_ear(&my_atoms);
        println!("ear: {:?}", ear);
        // if we can't find an ear, the query is not acyclic.
        if ear.0.is_none() {
            return None;
        // if the ear doesn't have a witness, add the ear to the last ear
        } else if ear.1.is_none() {
            // check if the ear is already in the join tree
            let mut idx_ear = None;
            for node in join_tree.node_indices() {
                if join_tree[node].same_vars(ear.0.unwrap()) {
                    idx_ear = Some(node);
                }
            }
            // if not, add it to the join tree
            if idx_ear == None {
                idx_ear = Some(join_tree.add_node(ear.0.unwrap().to_owned()));
            }
            // add an edge from the last ear to the current ear
            join_tree.add_edge(last_ear_idx, idx_ear.unwrap(), 0);
            last_ear_idx = idx_ear.unwrap();
            my_atoms = remove_atom(&my_atoms, ear.0.unwrap());
        // if the ear has a witness, add the witness and the ear to the join tree with an edge from witness to ear
        } else {
            let mut idx_ear = None;
            let mut idx_witness = None;
            // check if the witness and ear are already in the join tree
            for node in join_tree.node_indices() {
                if join_tree[node].same_vars(&ear.0.unwrap()) {
                    idx_ear = Some(node);
                }
                if join_tree[node].same_vars(&ear.1.unwrap()) {
                    idx_witness = Some(node);
                }
            }
            // if not, add them
            if idx_ear == None {
                idx_ear = Some(join_tree.add_node(ear.0.unwrap().to_owned()));
            }
            if idx_witness == None {
                idx_witness = Some(join_tree.add_node(ear.1.unwrap().to_owned()));
            }
            // add edge from witness to ear
            join_tree.add_edge(idx_witness.unwrap(), idx_ear.unwrap(), 0);
            last_ear_idx = idx_ear.unwrap();
            my_atoms = remove_atom(&my_atoms, ear.0.unwrap());
        }
    }
    return Some(join_tree);
}

// removes an atom from a vector of atoms
fn remove_atom(atoms: &Vec<Atom>, atom: &Atom) -> Vec<Atom> {
    let new_atoms: Vec<Atom> = atoms.to_vec().into_iter().filter(|a| a != atom).collect();
    return new_atoms.to_vec();
}

#[allow(dead_code)]
pub fn find_ear_old(atoms: &Vec<Atom>) -> (Option<&Atom>, Option<&Atom>) {
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

pub fn find_ear(atoms: &Vec<Atom>) -> (Option<&Atom>, Option<&Atom>) {
    // if there is only one atom, it is an ear
    if atoms.len() == 1 {
        return (Some(&atoms[0]), None);
    }
    // an atom is an ear if
    // 1 : all vertices of the hyperedge are exclusive to that hyperedge
    for atom in atoms {
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
    }
    // 2 : there is a witness for that atom
    for atom in atoms {
        let witness = find_witness_new(atoms, atom);
        if witness == None {
            continue;
        } else {
            return (Some(atom), witness);
        }
    }
    return (None, None);
}

pub fn find_witness_new<'a>(atoms: &'a Vec<Atom>, a: &Atom) -> Option<&'a Atom> {
    for potential_witness in atoms {
        let mut is_witness = true;
        if potential_witness == a {
            continue;
        } else {
            for term in &a.terms {
                // every term of the atom is either exclusive to the atom 
                // or it is also in the witness
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

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn yannakakis(query:Query, database: &mut DataBase) {
    // build a join tree from the query
    let join_tree = generate_join_tree(&query.body).unwrap();

    // find a root in the join tree
    let root = find_root(&join_tree).unwrap();

    // build a globally consistent database
    globally_consistent_database(database, &join_tree);

    // build a full reducer style, but with joins instead of semijoins
    let mut joins: Vec<NaturalJoin> = Vec::new();

    // bottom up traversal of the join tree to build first joins of the reducer
    post_order_apply(&join_tree, petgraph::graph::NodeIndex::new(root), &mut |join_tree, node| {
        // take the node from the join tree and find its parent
        let parent = join_tree
            .neighbors_directed(node, petgraph::Direction::Incoming)
            .next();
        // if the node has no parent, it is the root of the join tree 
        // nothing needs to be added to the reducer
        if parent == None {
            return;
        } else {
            // if the node has a parent, add a join to the reducer
            // the join has the node as its right child and the parent as its left child
            let join = NaturalJoin{
                left: join_tree[node].to_owned(),
                right: join_tree[parent.unwrap()].to_owned(),
            };
            joins.push(join);
        }
    });
    // print the joins
    for join in &joins {
        println!("{}", join);
    }

    // perform the joins
    for join in joins {
        naturaljoin(join, database, query.head.clone());
    }

}

#[allow(dead_code)]
pub fn globally_consistent_database(database: &mut DataBase, join_tree: &Graph<Atom, u8, Directed>) {
    let full_reducer = build_full_reducer_from_tree(join_tree);
    for semij in full_reducer {

        semijoin(&semij, database);
    }
}   

#[allow(dead_code)]
pub fn semijoin(semij: &SemiJoin, database: &mut DataBase) {
    // find the relation with the same name and arity as the left child of the semijoin
    let mut left_relation = None;
    for relation in &database.relations {
        if relation.name == semij.left.relation_name && relation.arity == semij.left.terms.len() {
            left_relation = Some(relation);
        }
    }
    // find the relation with the same name and arity as the right child of the semijoin
    let mut right_relation = None;
    for relation in &database.relations {
        if relation.name == semij.right.relation_name && relation.arity == semij.right.terms.len() {
            right_relation = Some(relation);
        }
    }
    // if one of the relations is not found, return
    if left_relation.is_none() || right_relation.is_none() {
        return;
    }
    // find indexes of common attributes of the two relations
    let mut common_attributes = Vec::new();
    for i in 0..left_relation.unwrap().arity {
        for j in 0..right_relation.unwrap().arity {
            if left_relation.unwrap().attributes[i] == right_relation.unwrap().attributes[j] {
                common_attributes.push((i, j));
            }
        }
    }
    // if there are no common attributes, return
    if common_attributes.len() == 0 {
        return;
    }
    // perform the semijoin based on the common attributes
    let mut new_tuples: Vec<Vec<ConstantTypes>> = Vec::new();

    // for every tuple in the left relation
    // look at every tuple in the right relation
    // if the common attributes are equal, add the tuple to the new relation
    // if it is not already in the new relation
    for left_tuple in &left_relation.unwrap().tuples {
        for right_tuple in &right_relation.unwrap().tuples {
            let mut common = true;
            for (i, j) in &common_attributes {
                if left_tuple[*i] != right_tuple[*j] {
                    common = false;
                    break;
                }
            }
            if common {
                if !new_tuples.contains(&left_tuple) {
                    new_tuples.push(left_tuple.clone());
                }
            }
        }
    }

    // make the new relation
    let new_left_relation = Relation{
        name: left_relation.unwrap().name.to_owned(),
        arity: left_relation.unwrap().arity.to_owned(),
        attributes: left_relation.unwrap().attributes.to_owned(),
        tuples: new_tuples,
    };
    // replace the old relation with the new one
    for relation in &mut database.relations {
        if relation.name == new_left_relation.name && relation.arity == new_left_relation.arity {
            *relation = new_left_relation;
            break;
        }
    }

    
}

pub fn naturaljoin(naturaljoin: NaturalJoin, database: &mut DataBase, projectionattributes: Vec<String>) {
    // find the relation with the same name and arity as the left child of the naturaljoin
    let mut left_relation = None;
    for relation in &database.relations {
        if relation.name == naturaljoin.left.relation_name {
            left_relation = Some(relation);
        }
    }
    // find the relation with the same name and arity as the right child of the naturaljoin
    let mut right_relation = None;
    for relation in &database.relations {
        if relation.name == naturaljoin.right.relation_name {
            right_relation = Some(relation);
        }
    }
    // if one of the relations is not found, return
    if left_relation.is_none() || right_relation.is_none() {
        return;
    }
    // find indexes of common attributes of the two relations
    let mut common_attributes = Vec::new();
    for i in 0..left_relation.unwrap().arity {
        for j in 0..right_relation.unwrap().arity {
            if left_relation.unwrap().attributes[i] == right_relation.unwrap().attributes[j] {
                common_attributes.push((i, j));
            }
        }
    }
    // if there are no common attributes, return
    if common_attributes.len() == 0 {
        return;
    }

    // compute total projection
    // the totalpojection is F ∪ (X ∩ E) for (E ⋈ F) with F parent of E
    let mut totalprojection: Vec<String> = Vec::new();

    // what is added by the intersection part
    let mut extraprojection: Vec<String> = Vec::new();
    let mut extraprojectionindex: Vec<usize> = Vec::new();

    // add the attributes of the right relation to the total projection
    for attribute in &right_relation.unwrap().attributes {
        totalprojection.push(attribute.to_owned());
    }
    // add the intersection of the projectionattributes and the left relation to the total projection
    for attribute in &left_relation.unwrap().attributes {
        if projectionattributes.contains(attribute) {
            if !totalprojection.contains(attribute) {
                totalprojection.push(attribute.to_owned());
                extraprojection.push(attribute.to_owned());
            }
        }
    }
    // get the extraprojection indexes
    for attribute in &extraprojection {
        for i in 0..left_relation.unwrap().attributes.len() {
            if attribute == &left_relation.unwrap().attributes[i] {
                extraprojectionindex.push(i);
            }
        }
    }
    // there are no extra attributes to add, return
    if extraprojection.len() == 0 {
        return;
    }
    // make the new relation
    let mut new_tuples: Vec<Vec<ConstantTypes>> = Vec::new();
    for left_tuple in &left_relation.unwrap().tuples {
        for right_tuple in &right_relation.unwrap().tuples {
            let mut common = true;
            for (i, j) in &common_attributes {
                if left_tuple[*i] != right_tuple[*j] {
                    common = false;
                    break;
                }
            }
            if common {
                let mut new_tuple = Vec::new();
                // add the attributes of the right relation to the new relation
                for attribute in right_tuple {
                    new_tuple.push(attribute.to_owned());
                }
                // add the extra projection to the new relation
                for i in &extraprojectionindex {
                    new_tuple.push(left_tuple[*i].to_owned());
                }
                // add the new tuple to the new relation
                new_tuples.push(new_tuple);
            }
        }
    }
    // make the new relation
    let new_relation = Relation{
        name: right_relation.unwrap().name.to_owned(),
        arity: totalprojection.len(),
        attributes: totalprojection,
        tuples: new_tuples,
    };
    // replace the old relation with the new one
    for relation in &mut database.relations {
        if relation.name == new_relation.name {
            *relation = new_relation;
            break;
        }
    }

    
}
