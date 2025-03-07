use std::fmt::Display;

pub fn encrypt(input: &str) -> String {
    Cipher::new(input).to_string()
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
        let check_sqrt = normal_length.sqrt().round() as usize;
        if check_sqrt.pow(2) >= normal_length as usize {
            (check_sqrt, check_sqrt)
        } else {
            (check_sqrt, check_sqrt + 1)
        }
    }
}

impl Display for Cipher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in 0..self.cols {
            if c != 0 {
                write!(f, " ")?;
            }
            for index in (c..=(self.rows - 1) * self.cols + c).step_by(self.cols) {
                write!(f, "{}", self.normal.get(index).unwrap_or(&' '))?;
            }
        }
        Ok(())
    }
}
