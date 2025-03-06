pub fn encrypt(input: &str) -> String {
    todo!("Encrypt {input:?} using a square code")
}

struct Cipher {
    rows: usize,
    cols: usize,
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

    fn determine_dimensions(normal_length: f64) -> (usize, usize) {
        let maybe_perfect_sqrt = normal_length.sqrt().round() as usize;
        if maybe_perfect_sqrt.pow(2) == normal_length as usize {
            (maybe_perfect_sqrt, maybe_perfect_sqrt)
        } else {
            let c = (0.25 + normal_length.sqrt() + 0.5).ceil() as usize;
            (c - 1, c)
        }
    }
}
