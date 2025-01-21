pub fn square(s: u32) -> u64 {
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    // 2_u128.pow(64) - 1 also works, but requires u128
    (1..=64).fold(0, |acc, s| square(s) + acc)
}
