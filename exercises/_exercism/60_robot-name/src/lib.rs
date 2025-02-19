// 26^2 * 10^3 = 676_000 possible sequences, each one totally characterized by
// their number. It is enough to get a single random int in that range.
use rand::{seq::IteratorRandom, thread_rng};
pub struct Robot {
    id: u32,
    name: String,
}

impl Robot {
    fn generate_id(exclude: &[u32]) -> (u32, String) {
        let identifier = (0_u32..676_000)
            .filter(|num| !exclude.contains(num))
            .choose(&mut thread_rng())
            .expect("There is not available names");
        let digits = identifier % 1000;
        let letter_1 = char::from_u32(identifier / 1000 % 26 + 65).unwrap();
        let letter_2 = char::from_u32(identifier / 26000 + 65).unwrap();
        (identifier, format!("{letter_1}{letter_2}{digits:0>3}"))
    }

    pub fn new() -> Self {
        let (id, name) = Self::generate_id(&[]);
        Self { id, name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        (self.id, self.name) = Self::generate_id(&[self.id]);
    }
}
