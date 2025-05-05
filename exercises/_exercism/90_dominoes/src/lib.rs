// The problem can be modeled as finding a Hamiltonian cycle in an
// undirected graph G, where each domino represents a node, and an edge
// connects two nodes if the corresponding stones have matching halves.
// In other words, this is a case of the Traveling Salesman Problem. This solution
// is based on the Held–Karp algorithm.
pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    todo!(
        "From the given input '{input:?}' construct a proper dominoes chain or return None if it is not possible."
    );
}

/// Returns the subsets as list of indices, subsets are ordered by size. The logic is
/// based in that given an indexed set S and its cardinality |S| = n, exist an
/// injective mapping from P(S) to the binary representation of the integers [0, 2^n-1].
fn subset_indices(cardinality: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::with_capacity(1 << cardinality); // 1 << n = 2^n
    for integer in 0_usize..(1 << cardinality) {
        let mut subset = vec![];
        for superset_index in 0..cardinality {
            if (integer >> superset_index) & 1 == 1 {
                subset.push(superset_index);
            }
        }
        result.push(subset);
    }
    result.sort_by_key(|v| v.len());
    result
}

fn edge_weight(node1: usize, node2: usize, input: &[(u8, u8)]) -> f64 {
    let ends = [input[node1].0, input[node1].1];
    if ends.contains(&input[node2].0) || ends.contains(&input[node2].1) {
        return 1.0;
    }
    f64::INFINITY
}
