pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .windows(len)
        .map(|w| w.join(""))
        .collect()
}
