pub fn get_diamond(letter: char) -> Vec<String> {
    Diamond::new(letter)
        .and_then(|mut d| {
            d.fill_shape();
            d.shape
        })
        .unwrap_or_default()
}

struct Diamond {
    letter_index: u8,
    dimension: usize,
    shape: Option<Vec<String>>,
}

impl Diamond {
    fn new(letter: char) -> Option<Self> {
        let letter_index = match letter {
            'A'..='Z' => letter as u8 - b'A',
            _ => return None,
        };
        Some(Self {
            letter_index,
            dimension: Self::letter_dim(letter_index),
            shape: None,
        })
    }

    fn letter_dim(letter_index: u8) -> usize {
        letter_index as usize * 2 - 1
    }

    fn fill_row(&mut self, row_index: u8) {
        todo!()
    }

    fn fill_shape(&mut self) {
        todo!()
    }
}
