use std::collections::HashSet;

// The problem can be modeled as finding a Hamiltonian cycle in an
// undirected graph G, where each domino represents a node, and an edge
// connects two nodes if the corresponding stones have matching halves.
// In other words, this is a case of the Traveling Salesman Problem.
pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    todo!(
        "From the given input '{input:?}' construct a proper dominoes chain or return None if it is not possible."
    );
}
