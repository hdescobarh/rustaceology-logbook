use std::collections::{HashMap, HashSet};

/// The problem can be modeled as finding and Eulerian cycle where each stone is an edge
pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    PseudoMultiGraph::from_edges(input).find_eulerian_cycle()
}

/// A symmetric graph-like structure which allows self-edges and multiple edges between a pair of nodes
struct PseudoMultiGraph {
    /// a map node_i ↦ (node_j, multiplicity_j) s.t. degree(node_i) > 0
    adjacency: HashMap<u8, HashMap<u8, usize>>,
    nodes: HashSet<u8>,
}

impl PseudoMultiGraph {
    fn from_edges(input: &[(u8, u8)]) -> Self {
        let mut adjacency: HashMap<u8, HashMap<u8, usize>> = HashMap::new();
        let mut nodes = HashSet::new();
        for (u, v) in input.iter().flat_map(|(u, v)| [(u, v), (v, u)]) {
            *adjacency.entry(*u).or_default().entry(*v).or_default() += 1;
            nodes.insert(*u);
        }
        Self { adjacency, nodes }
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

    /// given a node_i, gets an adjacent node_j, given priority to node_i = node_j
    fn get_adjacent(&self, node: &u8) -> Option<u8> {
        let map = &self.adjacency[node];
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
            .any(|map| map.values().sum() % 2 != 0)
        {
            return None;
        }

        let mut node_cycles: Vec<Vec<u8>> = vec![];
        while !self.adjacency.is_empty() {
            let parent = match self.adjacency.iter().find(|(_node, map)| !map.is_empty()) {
                Some((node, _degree)) => *node,
                None => return Some(vec![]),
            };
            node_cycles.push(self.deep_first_search(parent))
        }
        let mut edge_cycle = vec![];
        let mut node_cycle_iter = Self::try_merge_cycles(&node_cycles)?;

        if let Some(mut start) = node_cycle_iter.next() {
            for end in node_cycle_iter {
                edge_cycle.push((*start, *end));
                start = end;
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

    fn try_merge_cycles(paths: &[Vec<u8>]) -> Option<impl Iterator<Item = &u8>> {
        let mut result: Vec<&[u8]> = paths
            .first()
            .map(|v| vec![v.as_slice()])
            .unwrap_or_default();
        let mut pending_pos: HashSet<usize> = (1..paths.len()).collect();

        while !pending_pos.is_empty() {
            let mut complete_iter_without_share = true;
            for (insert_pos, insert) in pending_pos.iter().map(|&i| (i, &paths[i])) {
                let shared_node = result.iter().enumerate().find_map(|(outer_pos, sub_path)| {
                    sub_path
                        .iter()
                        .position(|node| *node == insert[0])
                        .map(|inner_pos| (outer_pos, inner_pos))
                });

                if shared_node.is_none() {
                    continue;
                }

                let (outer_pos, inner_pos) = shared_node.unwrap();
                result[outer_pos] = &result[outer_pos][inner_pos..];
                result.splice(
                    outer_pos..outer_pos,
                    [&result[outer_pos][..inner_pos], insert],
                );

                complete_iter_without_share = false;
                pending_pos.remove(&insert_pos);
                break;
            }

            if complete_iter_without_share {
                return None;
            }
        }
        Some(result.into_iter().flat_map(|inner| inner.iter()))
    }
}
