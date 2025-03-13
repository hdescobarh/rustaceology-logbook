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

    pub fn decode(&self, cipher: &str) -> String {
        let cipher_chars: Vec<char> = cipher.chars().collect();
        (0..cipher.len())
            .fold(vec![Vec::<usize>::new(); self.rails], |mut acc, index| {
                acc[self.plain_index_to_row(index)].push(index);
                acc
            })
            .into_iter()
            .flatten()
            .enumerate()
            .fold(
                vec![' '; cipher.len()],
                |mut acc, (cipher_index, plain_index)| {
                    acc[plain_index] = cipher_chars[cipher_index];
                    acc
                },
            )
            .into_iter()
            .collect()
    }
}
