use std::fmt::Write;
const DICT: [(u32, &str); 3] = [(3, "i"), (5, "a"), (7, "o")];
pub fn raindrops(n: u32) -> String {
    let mut buffer = String::new();
    DICT.iter().for_each(|(d, c)| {
        if n % d == 0 {
            write!(&mut buffer, "Pl{}ng", c).unwrap();
        }
    });

    if buffer.is_empty() {
        n.to_string()
    } else {
        buffer
    }
}
