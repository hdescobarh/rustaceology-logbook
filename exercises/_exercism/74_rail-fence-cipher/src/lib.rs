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

    pub fn encode(&self, text: &str) -> String {
        text.char_indices()
            .fold(
                vec![String::new(); self.rails],
                |mut acc, (index, letter)| {
                    let cycle_position = index % self.period;
                    let row = match cycle_position.cmp(&self.rails) {
                        std::cmp::Ordering::Less => cycle_position,
                        _ => self.period - cycle_position,
                    };
                    acc[row].push(letter);
                    acc
                },
            )
            .join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        todo!("Decode this ciphertext: {cipher}")
    }
}
