use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug)]
pub struct CustomSet<T: PartialEq + Eq + Hash + Clone + Copy> {
    buckets: Vec<Option<Vec<T>>>,
    capacity: usize,
    size: usize,
}

impl<T: PartialEq + Eq + Hash + Clone + Copy> CustomSet<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buckets: vec![None::<Vec<T>>; capacity],
            capacity,
            size: 0,
        }
    }
    pub fn new(input: &[T]) -> Self {
        let mut set = Self::with_capacity(input.len());
        for &value in input {
            let index = set.hash(&value);
            match set.buckets[index].as_mut() {
                Some(list) if list.contains(&value) => (),
                Some(list) => {
                    list.push(value);
                    set.size += 1;
                }
                None => {
                    set.buckets[index] = Some(vec![value]);
                    set.size += 1;
                }
            };
        }
        set
    }

    fn hash(&self, value: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    pub fn contains(&self, _element: &T) -> bool {
        todo!();
    }

    pub fn add(&mut self, _element: T) {
        // update indexes
        // update size & capacity
        todo!();
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        todo!();
    }

    pub fn is_empty(&self) -> bool {
        todo!();
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        todo!();
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        todo!();
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        todo!();
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        todo!();
    }
}
