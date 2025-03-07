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

    // From the constraints:
    // - Option 1: c_1 = round(sqrt(n)), with r = c_1
    // - Option 2: c_2 = ceil(0.5 + sqrt(0.25 + n)), with r = c_1 - 1
    // Since min(c_1, c_2) = c_1, the choice depends on whether c_1^2 >= n.
    //
    // I haven't found a formal proof, but I suspect that c_2 = c_1 + 1.
    // I verified this holds for the first 10^9 integers.
    fn determine_dimensions(normal_length: f64) -> (usize, usize) {
        let check_sqrt = normal_length.sqrt().round() as usize;
        if check_sqrt.pow(2) >= normal_length as usize {
            (check_sqrt, check_sqrt)
        } else {
            (check_sqrt, check_sqrt + 1)
        }
    }
}
