pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        // we expect only ASCII, so its is ok to compare the length in bytes
        return None;
    }
    Some(
        s1.chars()
            .zip(s2.chars())
            .fold(0_usize, |acc, (c1, c2)| acc + if c1 != c2 { 1 } else { 0 }),
    )
}
