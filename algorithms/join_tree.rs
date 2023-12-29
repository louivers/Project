use crate::algorithms::gyo::gyo_return_steps;
use crate::models::query::{Query, Atom};
use petgraph::{Graph, Directed};

// generates a join tree based on the steps taken by the gyo algorithm
pub fn generate_join_tree(atoms: &Vec<Atom>) -> Option<Graph<Atom, u8, Directed>> {
    let gyo_res = gyo_return_steps(&Query{head: Vec::new(), body: atoms.to_vec()});
    if gyo_res.0 {
        let steps = gyo_res.1;
        let mut join_tree = Graph::<Atom, u8, Directed>::new();
        for step in steps {
            let mut idx_ear = None;
            let mut idx_witness = None;
            // check if the witness and ear are already in the join tree
            for node in join_tree.node_indices() {
                if join_tree[node].same_vars(&step.0) {
                    idx_ear = Some(node);
                }
                if join_tree[node].same_vars(&step.1) {
                    idx_witness = Some(node);
                }
            }
            // if not, add them
            if idx_ear == None {
                idx_ear = Some(join_tree.add_node(step.0.to_owned()));
            }
            if idx_witness == None {
                idx_witness = Some(join_tree.add_node(step.1.to_owned()));
            }
            // add edge from witness to ear
            join_tree.add_edge(idx_witness.unwrap(), idx_ear.unwrap(), 0);
        }
        return Some(join_tree);

    }
    return None;
        
}

