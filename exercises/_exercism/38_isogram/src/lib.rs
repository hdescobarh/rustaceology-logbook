use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();
    !candidate
        .chars()
        .filter(|c| ![' ', '-'].contains(c))
        .any(|c| !letters.insert(c.to_ascii_lowercase()))
}
