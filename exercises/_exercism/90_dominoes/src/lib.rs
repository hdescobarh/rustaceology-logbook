use core::f64;
use std::collections::HashMap;

// The problem can be modeled as finding a Hamiltonian cycle in an
// undirected graph G, where each domino represents a node, and an edge
// connects two nodes if the corresponding stones have matching halves.
// In other words, this is a case of the Traveling Salesman Problem. This solution
// is based on the Held–Karp algorithm.
pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    match input.len() {
        0 => return Some(input.to_vec()),
        1 if input[0].0 == input[0].1 => return Some(input.to_vec()),
        1 => return None,
        _ => (),
    }
    let partition = &input[1..];
    let power_set = PowerSet::from_cardinality(partition.len());
    let mut cost_and_precursors: HashMap<(usize, usize), (f64, (usize, usize))> =
        HashMap::with_capacity(2 * (1 << partition.len()));
    for (subset_k, node_i) in power_set.iter_singletons() {
        cost_and_precursors.insert((node_i, subset_k), (edge_weight(0, node_i, input), (0, 0)));
    }

    for (subset_k, elements) in power_set.iter_non_singletons() {
        for &node_i in elements {
            let mut min_cost = f64::INFINITY;
            let subset_k_excluding_node_i = subset_k & !(1 << node_i);
            for &node_j in &power_set.subset_elements[&subset_k_excluding_node_i] {
                let current_cost = cost_and_precursors[&(node_j, subset_k_excluding_node_i)].0
                    + edge_weight(node_i, node_j, input);
                if min_cost > current_cost {
                    min_cost = current_cost;
                }
            }
            cost_and_precursors.insert((node_i, subset_k), (min_cost, (0, 0)));
        }
    }

    let mut min_cost = f64::INFINITY;
    let subset_k = *power_set.size_to_subsets.last().unwrap().last().unwrap();
    for &node_i in &power_set.subset_elements[&subset_k] {
        let current_cost =
            cost_and_precursors[&(node_i, subset_k)].0 + edge_weight(0, node_i, input);
        if min_cost > current_cost {
            min_cost = current_cost;
        }
    }

    if min_cost.is_infinite() {
        return None;
    }

    todo!(
        "From the given input '{input:?}' construct a proper dominoes chain or return None if it is not possible."
    );
}

fn edge_weight(node1: usize, node2: usize, input: &[(u8, u8)]) -> f64 {
    let ends = [input[node1].0, input[node1].1];
    if ends.contains(&input[node2].0) || ends.contains(&input[node2].1) {
        return 1.0;
    }
    f64::INFINITY
}

struct PowerSet {
    size_to_subsets: Vec<Vec<usize>>,
    subset_elements: HashMap<usize, Vec<usize>>,
}

impl PowerSet {
    /// The logic is based in that given an indexed set S and its cardinality |S| = n, exist an
    /// injective mapping from P(S) to the binary representation of the integers [0, 2^n-1].
    fn from_cardinality(value: usize) -> Self {
        let mut size_to_subsets: Vec<Vec<usize>> = vec![vec![]; value + 1];
        let mut subset_elements: HashMap<usize, Vec<usize>> = HashMap::with_capacity(1 << value); // 2^n = 1 << n
        for subset_id in 0_usize..(1 << value) {
            let mut elements = vec![];
            for superset_index in 0..value {
                if (subset_id >> superset_index) & 1 == 1 {
                    elements.push(superset_index);
                }
            }
            size_to_subsets[elements.len()].push(subset_id);
            subset_elements.insert(subset_id, elements);
        }

        Self {
            size_to_subsets,
            subset_elements,
        }
    }

    fn iter_singletons(&self) -> impl Iterator<Item = (usize, usize)> {
        self.size_to_subsets[1]
            .iter()
            .map(|id| (*id, *self.subset_elements[id].first().unwrap()))
    }
    fn iter_non_singletons(&self) -> impl Iterator<Item = (usize, &Vec<usize>)> {
        self.size_to_subsets
            .iter()
            .skip(2)
            .flatten()
            .map(|id| (*id, &self.subset_elements[id]))
    }
}
