use std::collections::HashMap;

/// The problem can be modeled as finding and Eulerian cycle where each stone is an edge
pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    todo!(
        "From the given input '{input:?}' construct a proper dominoes chain or return None if it is not possible."
    );
}

/// A symmetric graph-like structure which allows self-edges and multiple edges between a pair of nodes
struct PseudoMultiGraph {
    /// a map node_i ↦ (node_j, multiplicity_j)
    adjacency: HashMap<u8, HashMap<u8, usize>>,
}

impl PseudoMultiGraph {
    fn from_edges(input: &[(u8, u8)]) -> Self {
        let mut adjacency: HashMap<u8, HashMap<u8, usize>> = HashMap::new();
        for (u, v) in input.iter().flat_map(|(u, v)| [(u, v), (v, u)]) {
            *adjacency.entry(*u).or_default().entry(*v).or_default() += 1
        }
        Self { adjacency }
    }

    fn remove_edge(&mut self, node1: u8, node2: u8) {
        for (u, v) in [(node1, node2), (node2, node1)] {
            let multiset = self.adjacency.get_mut(&u).unwrap();
            if multiset[&v] == 1 {
                multiset.remove_entry(&v);
            } else {
                *multiset.get_mut(&v).unwrap() -= 1;
            }
        }
    }

    /// given a node_i, gets an adjacent node_j, given priority to node_i = node_j
    fn get_adjacent(&self, node: &u8) -> Option<u8> {
        let map = &self.adjacency[node];
        if map.get(node).is_some() {
            return Some(*node);
        }
        map.keys().next().copied()
    }

    /// returns an iterator over the nodes degree
    fn iter_degree(&self) -> impl Iterator<Item = (&u8, usize)> {
        self.adjacency
            .iter()
            .map(|(node, map)| (node, map.values().sum()))
    }
}
