// 26^2 * 10^3 = 676_000 possible sequences, each one totally characterized by
// their number. It is enough to get a single random int in that range.
use rand::Rng;
use std::{
    collections::HashSet,
    sync::{LazyLock, Mutex},
};

static ID_REGISTRY: LazyLock<Mutex<HashSet<u32>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub struct Robot {
    id: u32,
    name: String,
}

impl Robot {
    fn generate_id(excluded: &mut HashSet<u32>) -> (u32, String) {
        if excluded.len() >= 676_000 {
            panic!("There is not available names")
        }
        let mut rng = rand::thread_rng();
        let mut id = rng.gen_range(0_u32..676_000);
        while !excluded.insert(id) {
            id = rng.gen_range(0_u32..676_000);
        }
        let digits = id % 1000;
        let letter_1 = char::from_u32(id / 1000 % 26 + 65).unwrap();
        let letter_2 = char::from_u32(id / 26000 + 65).unwrap();
        (id, format!("{letter_2}{letter_1}{digits:0>3}"))
    }

    pub fn new() -> Self {
        let (id, name) = Self::generate_id(&mut ID_REGISTRY.lock().unwrap());
        Self { id, name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        let old_id = self.id;
        let registry = &mut ID_REGISTRY.lock().unwrap();
        (self.id, self.name) = Self::generate_id(registry);
        registry.remove(&old_id);
    }
}
