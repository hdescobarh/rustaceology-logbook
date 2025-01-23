use std::collections::HashSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|base_value| **base_value != 0)
        .flat_map(|base_value| (*base_value..limit).step_by(*base_value as usize))
        .collect::<HashSet<u32>>()
        .iter()
        .sum()
}
