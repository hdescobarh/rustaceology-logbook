pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(|c| {
            if !c.is_ascii_alphabetic() {
                return c;
            }
            let shift = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            ((c as u8 - shift + key) % 26 + shift) as char
        })
        .collect()
}
