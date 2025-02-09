pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(Vec::with_capacity(row_count as usize)).fill(row_count as usize)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }

    fn fill(mut self, row_count: usize) -> Self {
        while self.0.len() < row_count {
            if self.0.is_empty() {
                self.0.push(vec![1]);
                continue;
            }
            let next_row: Vec<u32> = [1]
                .into_iter()
                .chain(self.0.last().unwrap().windows(2).map(|v| v[0] + v[1]))
                .chain([1].into_iter())
                .collect();
            self.0.push(next_row)
        }
        self
    }
}
