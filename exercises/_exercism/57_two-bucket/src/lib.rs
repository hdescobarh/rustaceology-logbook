use std::{collections::HashMap, hash::Hash};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    todo!(
        "Given one bucket of capacity {capacity_1}, another of capacity {capacity_2}, starting with {start_bucket:?}, find pours to reach {goal}, or None if impossible"
    );
}

struct TreeNode<'a> {
    path_length: u8,
    capacity_1: &'a u8,
    capacity_2: &'a u8,
    volume_1: u8,
    volume_2: u8,
}

impl<'a> TreeNode<'a> {
    pub fn new(capacity_1: &'a u8, capacity_2: &'a u8, start_bucket: &Bucket) -> Self {
        let (volume_1, volume_2) = match start_bucket {
            Bucket::One => (*capacity_1, 0),
            Bucket::Two => (0, *capacity_2),
        };
        Self {
            path_length: 1,
            capacity_1,
            capacity_2,
            volume_1,
            volume_2,
        }
    }

    pub fn branch(node: Option<TreeNode>, goal: u8, visited: &mut HashMap<State, u8>) {
        let node = match node {
            Some(node) => node,
            None => return,
        };
        let state = State::get_from_node(&node);
        match visited.get_mut(&state) {
            Some(length) if node.path_length < *length => *length = node.path_length,
            Some(_) => return,
            None => {
                let new_length = node.path_length;
                visited.insert(state, new_length);
            }
        };

        if state.arrived_to_goal(goal) {
            return;
        }

        Self::branch(node.fill(&Bucket::One), goal, visited);
        Self::branch(node.fill(&Bucket::Two), goal, visited);
        Self::branch(node.empty(&Bucket::One), goal, visited);
        Self::branch(node.empty(&Bucket::Two), goal, visited);
        Self::branch(node.pour(&Bucket::Two, &Bucket::One), goal, visited);
        Self::branch(node.pour(&Bucket::One, &Bucket::Two), goal, visited);
    }

    fn fill(&self, bucket: &Bucket) -> Option<Self> {
        let (volume_1, volume_2) = match bucket {
            Bucket::One if self.volume_1 != *self.capacity_1 => (*self.capacity_1, self.volume_2),
            Bucket::Two if self.volume_2 != *self.capacity_2 => (self.volume_1, *self.capacity_2),
            _ => return None,
        };
        Some(Self {
            path_length: self.path_length + 1,
            capacity_1: self.capacity_1,
            capacity_2: self.capacity_2,
            volume_1,
            volume_2,
        })
    }

    fn empty(&self, bucket: &Bucket) -> Option<Self> {
        let (volume_1, volume_2) = match bucket {
            Bucket::One if self.volume_1 != 0 => (0, self.volume_2),
            Bucket::Two if self.volume_2 != 0 => (self.volume_1, 0),
            _ => return None,
        };
        Some(Self {
            path_length: self.path_length + 1,
            capacity_1: self.capacity_1,
            capacity_2: self.capacity_2,
            volume_1,
            volume_2,
        })
    }

    fn pour(&self, from: &Bucket, into: &Bucket) -> Option<Self> {
        let (from_volume, into_volume, into_capacity) = match (from, into) {
            (Bucket::One, Bucket::Two) if self.volume_1 > 0 && self.volume_2 < *self.capacity_2 => {
                (self.volume_1, self.volume_2, self.capacity_2)
            }
            (Bucket::Two, Bucket::One) if self.volume_2 > 0 && self.volume_1 < *self.capacity_1 => {
                (self.volume_2, self.volume_1, self.capacity_1)
            }
            _ => return None,
        };
        let remaining_capacity = into_capacity - into_volume;
        let (from_volume, into_volume) = match from_volume.cmp(&remaining_capacity) {
            std::cmp::Ordering::Less => (0, from_volume + into_volume),
            std::cmp::Ordering::Equal => (0, into_volume),
            std::cmp::Ordering::Greater => (from_volume - remaining_capacity, into_volume),
        };

        let (volume_1, volume_2) = match from {
            Bucket::One => (from_volume, into_volume),
            Bucket::Two => (into_volume, from_volume),
        };

        Some(Self {
            path_length: self.path_length + 1,
            capacity_1: self.capacity_1,
            capacity_2: self.capacity_2,
            volume_1,
            volume_2,
        })
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct State {
    volume_1: u8,
    volume_2: u8,
}

impl State {
    fn get_from_node(node: &TreeNode) -> Self {
        Self {
            volume_1: node.volume_1,
            volume_2: node.volume_2,
        }
    }

    fn arrived_to_goal(&self, goal: u8) -> bool {
        self.volume_1 == goal || self.volume_2 == goal
    }
}
