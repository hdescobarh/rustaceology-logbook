#[derive(Debug)]
pub struct HighScores {
    values: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            values: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.values
    }

    pub fn latest(&self) -> Option<u32> {
        self.values.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.values.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut to_sort = self.values.clone();
        to_sort.sort_by(|a, b| b.cmp(a));
        to_sort.truncate(3);
        to_sort
    }
}
