use std::collections::{HashMap, HashSet};

/// The problem can be modeled as finding an Eulerian cycle, where each stone represents an edge.
pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    PseudoMultiGraph::from_edges(input).find_eulerian_cycle()
}

/// A symmetric, graph-like structure that allows self-edges and multiple edges between pairs of nodes.
struct PseudoMultiGraph {
    /// a map node_i ↦ (node_j, multiplicity_j) s.t. degree(node_i) > 0
    adjacency: HashMap<u8, HashMap<u8, usize>>,
    _nodes: HashSet<u8>,
}

impl PseudoMultiGraph {
    fn from_edges(input: &[(u8, u8)]) -> Self {
        let mut adjacency: HashMap<u8, HashMap<u8, usize>> = HashMap::new();
        let mut _nodes = HashSet::new();
        for (u, v) in input.iter().flat_map(|(u, v)| [(u, v), (v, u)]) {
            *adjacency.entry(*u).or_default().entry(*v).or_default() += 1;
            _nodes.insert(*u);
        }
        Self { adjacency, _nodes }
    }

    fn remove_edge(&mut self, node1: u8, node2: u8) {
        for (u, v) in [(node1, node2), (node2, node1)] {
            match (self.adjacency[&u].len(), self.adjacency[&u][&v]) {
                (1, 1) => {
                    self.adjacency.remove_entry(&u);
                }
                (_, 1) => {
                    self.adjacency.get_mut(&u).unwrap().remove_entry(&v);
                }
                _ => *self.adjacency.get_mut(&u).unwrap().get_mut(&v).unwrap() -= 1,
            }
        }
    }

    //// Given a node `i`, returns an adjacent node `j`, giving priority to the case where `i == j`.
    fn get_adjacent(&self, node: &u8) -> Option<u8> {
        let map = &self.adjacency.get(node)?;
        if map.get(node).is_some() {
            return Some(*node);
        }
        map.keys().next().copied()
    }

    fn find_eulerian_cycle(&mut self) -> Option<Vec<(u8, u8)>> {
        // An undirected graph can contain an Eulerian cycle only if all its nodes have even degree.
        if self
            .adjacency
            .values()
            .any(|map| map.values().sum::<usize>() % 2 != 0)
        {
            return None;
        }

        let mut node_cycle: Vec<u8> = match self.adjacency.keys().next() {
            Some(node) => self.deep_first_search(*node),
            None => return Some(vec![]),
        };

        while !self.adjacency.is_empty() {
            // If there are no shared nodes, the graph is not connected, so no Eulerian path exists.
            let shared_node_pos = node_cycle
                .iter()
                .position(|node| self.adjacency.contains_key(node))?;
            let next_cycle = self.deep_first_search(node_cycle[shared_node_pos]);
            node_cycle.splice(shared_node_pos..shared_node_pos + 1, next_cycle);
        }

        let mut edge_cycle = Vec::with_capacity(node_cycle.len());
        for window in node_cycle.windows(2) {
            match window {
                [a, b] => edge_cycle.push((*a, *b)),
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
}
