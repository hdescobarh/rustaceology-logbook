pub struct RailFence {
    rails: usize,
    period: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self {
            rails: rails as usize,
            period: (2 * rails - 2) as usize,
        }
    }

    fn plain_index_to_row(&self, index: usize) -> usize {
        let cycle_position = index % self.period;
        match cycle_position.cmp(&self.rails) {
            std::cmp::Ordering::Less => cycle_position,
            _ => self.period - cycle_position,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        text.char_indices()
            .fold(
                vec![String::new(); self.rails],
                |mut acc, (index, letter)| {
                    acc[self.plain_index_to_row(index)].push(letter);
                    acc
                },
            )
            .join("")
    }

    fn cipher_to_plain_indices(&self, length: usize) -> impl Iterator<Item = (usize, usize)> {
        let mut mapping = vec![Vec::with_capacity(2 * length.div_ceil(self.period)); self.rails];
        for index in 0..length {
            mapping[self.plain_index_to_row(index)].push(index)
        }
        mapping.into_iter().flatten().enumerate()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let cipher_chars: Vec<char> = cipher.chars().collect();
        self.cipher_to_plain_indices(cipher_chars.len())
            .fold(
                vec![' '; cipher_chars.len()],
                |mut acc, (cipher_index, plain_index)| {
                    acc[plain_index] = cipher_chars[cipher_index];
                    acc
                },
            )
            .into_iter()
            .collect()
    }
}
