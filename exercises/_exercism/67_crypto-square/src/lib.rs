pub fn encrypt(input: &str) -> String {
    todo!("Encrypt {input:?} using a square code")
}

struct Cipher {
    rows: u64,
    cols: u64,
    normal: Vec<char>,
}

impl Cipher {
    fn new(plain_text: &str) -> Self {
        let normal: Vec<char> = plain_text
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let (rows, cols) = Self::determine_dimensions(normal.len() as f64);
        Self { rows, cols, normal }
    }

    fn determine_dimensions(normal_length: f64) -> (u64, u64) {
        let maybe_perfect_sqrt = normal_length.sqrt().round() as u64;
        if maybe_perfect_sqrt.pow(2) == normal_length as u64 {
            (maybe_perfect_sqrt, maybe_perfect_sqrt)
        } else {
            let c = (0.25 + normal_length.sqrt() + 0.5).ceil() as u64;
            (c - 1, c)
        }
    }
}
