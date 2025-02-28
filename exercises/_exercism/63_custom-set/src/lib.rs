use std::hash::{DefaultHasher, Hash, Hasher};
const MAX_LOAD_FACTOR: f64 = 0.75; // remember α cannot exceed 1

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

    fn load_factor(&self) -> Option<f64> {
        if self.capacity == 0 {
            None
        } else {
            Some(self.size as f64 / self.capacity as f64)
        }
    }

    fn resize(&mut self) {
        let mut set = Self::with_capacity(self.capacity * 2);
        todo!()
    }

    pub fn contains(&self, element: &T) -> bool {
        if self.is_empty() {
            return false;
        }
        match self.buckets[self.hash(element)].as_ref() {
            Some(list) => list.contains(element),
            None => false,
        }
    }

    pub fn add(&mut self, _element: T) {
        self.capacity = if self.capacity == 0 {
            (1.0 / MAX_LOAD_FACTOR).ceil() as usize
        } else {
            self.capacity * 2
        };
        todo!();
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        // self ⊆ other ⟺ for all x, self.contains(x); then other.contains(x)
        // ∅ is subset of every set
        self.buckets
            .iter()
            .flatten()
            .all(|bucket| bucket.iter().all(|element| other.contains(element)))
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        // A ∩ B = ∅
        self.intersection(other).is_empty()
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        // A ∩ B = {x | x ∈ A ∧ x ∈ B}
        let shared_elements = self
            .buckets
            .iter()
            .flatten()
            .flatten()
            .filter_map(|element| {
                if other.contains(element) {
                    Some(*element)
                } else {
                    None
                }
            })
            .collect::<Vec<T>>();
        Self::new(&shared_elements)
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        // A \ B = {x | x ∈ A ∧ x ∉ B}
        let self_only_elements = self
            .buckets
            .iter()
            .flatten()
            .flatten()
            .filter_map(|element| {
                if other.contains(element) {
                    None
                } else {
                    Some(*element)
                }
            })
            .collect::<Vec<T>>();
        Self::new(&self_only_elements)
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        // A ∪ B = {x | x ∈ A ∨ x ∈ B}
        let all_elements = self
            .buckets
            .iter()
            .flatten()
            .flatten()
            .chain(other.buckets.iter().flatten().flatten())
            .copied()
            .collect::<Vec<T>>();
        Self::new(&all_elements)
    }
}

// A = B ⟺ A ⊆ B ∧ A ⊇ B
impl<T> PartialEq for CustomSet<T>
where
    T: PartialEq + Eq + Hash + Clone + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self) && self.size == other.size
    }
}

impl<T> Eq for CustomSet<T> where T: PartialEq + Eq + Hash + Clone + Copy {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn max_load_factor_is_valid() {
        if MAX_LOAD_FACTOR > 1.0 {
            panic!("MAX_LOAD_FACTOR cannot exceed 1.0")
        }
    }
}
