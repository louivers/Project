use crate::models::query::{Relation, SemiJoin, NaturalJoin, DataBase, ConstantTypes};

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