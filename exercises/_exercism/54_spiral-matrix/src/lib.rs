// Given a matrix mxn the spiral movement follows this pattern (steps direction):
// n →, m-1 ↓, n-1 ←,  m-2 ↑, n-2 →, ... continues decreasing the number of steps
// in each direction by 1 in each cycle, until all the nm cells have been visited.
// This means, the sequence can end in two ways:
// (a) ..., m-k, n-k; k = m-1 if m <= n
// (b) ..., n-(k-1), m-k; k = n if m > n
// The total number of steps is given by:
// f(k) = n + \sum_{i=1}^k (m-i) + \sum_{i=1}^k (n-i) =
// n + km + kn - k(k+1) = n + k (m+n-k-1)
// Replacing k why should get mn:
// (a) f(m-1) = mn
// (b) f(n) = mn
// Then, the strategy is to implement a generator of the pattern
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    todo!("Function that returns the spiral matrix of square size {size}");
}
