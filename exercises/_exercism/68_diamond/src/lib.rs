pub fn get_diamond(letter: char) -> Vec<String> {
    Diamond::new(letter)
        .and_then(|mut d| {
            d.fill_shape();
            d.shape
        })
        .unwrap_or_default()
}

struct Diamond {
    letter_index: usize,
    dimension: usize,
    shape: Option<Vec<String>>,
}

impl Diamond {
    fn new(letter: char) -> Option<Self> {
        let letter_index = match letter {
            'A'..='Z' => (letter as u8 - b'A') as usize,
            _ => return None,
        };
        Some(Self {
            letter_index,
            dimension: Self::letter_dim(letter_index),
            shape: None,
        })
    }

    fn letter_dim(letter_index: usize) -> usize {
        letter_index * 2 + 1
    }

    fn make_row(&self, row_index: usize) -> String {
        let dim = Self::letter_dim(row_index);
        let inner = dim.saturating_sub(2);
        let outer = (self.dimension - dim) / 2;
        if row_index == 0 {
            format!("{s:outer$}A{s:outer$}", s = "")
        } else {
            format!(
                "{s:outer$}{letter}{s:inner$}{letter}{s:outer$}",
                s = "",
                letter = (row_index as u8 + b'A') as char
            )
        }
    }

    fn fill_shape(&mut self) {
        let mut shape: Vec<String> = Vec::with_capacity(self.dimension);
        for row_index in 0..=self.letter_index {
            let row = self.make_row(row_index);
            shape.push(row);
        }
        shape.extend_from_within(..self.letter_index);
        shape[self.letter_index + 1..].reverse();
        self.shape = Some(shape)
    }
}
