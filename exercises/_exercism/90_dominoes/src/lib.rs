use std::collections::HashMap;

/// The problem can be modeled as finding and Eulerian cycle where each stone is an edge
pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    PseudoMultiGraph::from_edges(input).find_eulerian_cycle()
}

/// A symmetric graph-like structure which allows self-edges and multiple edges between a pair of nodes
struct PseudoMultiGraph {
    /// a map node_i ↦ (node_j, multiplicity_j)
    adjacency: HashMap<u8, HashMap<u8, usize>>,
    edges_size: usize,
}

impl PseudoMultiGraph {
    fn from_edges(input: &[(u8, u8)]) -> Self {
        let mut adjacency: HashMap<u8, HashMap<u8, usize>> = HashMap::new();
        for (u, v) in input.iter().flat_map(|(u, v)| [(u, v), (v, u)]) {
            *adjacency.entry(*u).or_default().entry(*v).or_default() += 1
        }
        Self {
            adjacency,
            edges_size: input.len(),
        }
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
        self.edges_size -= 1;
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

    fn find_eulerian_cycle(&mut self) -> Option<Vec<(u8, u8)>> {
        if self.iter_degree().any(|(_node, degree)| degree % 2 != 0) {
            return None;
        }
        let mut node_cycles: Vec<Vec<u8>> = vec![];

        while self.edges_size > 0 {
            let parent = match self.iter_degree().find(|(_node, degree)| *degree != 0) {
                Some((node, _degree)) => *node,
                None => return Some(vec![]),
            };
            node_cycles.push(self.deep_first_search(parent))
        }
        let mut edge_cycle = vec![];
        for window in Self::try_connect_cycles(node_cycles)?.windows(2) {
            match window {
                [u, v] => edge_cycle.push((*u, *v)),
                _ => return None,
            }
        }
        Some(edge_cycle)
    }

    fn deep_first_search(&mut self, mut parent: u8) -> Vec<u8> {
        let mut node_path: Vec<u8> = vec![];
        while let Some(child) = self.get_adjacent(&parent) {
            self.remove_edge(parent, child);
            node_path.push(parent);
            parent = child;
        }
        node_path.push(parent);
        node_path
    }

    fn try_connect_cycles(mut cycles: Vec<Vec<u8>>) -> Option<Vec<u8>> {
        todo!()
    }
}
