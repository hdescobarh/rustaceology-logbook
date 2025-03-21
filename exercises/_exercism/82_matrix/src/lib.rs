use std::num::ParseIntError;

#[derive(Default)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    inner: Vec<u32>, // Avoid unnecessary layers of indirection that Vec<Vec<T>> has.
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        Self::validate(input).unwrap_or_default()
    }

    // Not required by the exercise, but it feels odd to not check if the input is valid 😊.
    fn validate(input: &str) -> Option<Self> {
        let data = input
            .lines()
            .map(|row| {
                row.split_whitespace()
                    .map(|e| e.parse::<u32>())
                    .collect::<Result<Vec<u32>, ParseIntError>>()
            })
            .collect::<Result<Vec<Vec<u32>>, ParseIntError>>()
            .ok()?;
        let cols = data.first().and_then(|first_row| {
            if data.iter().all(|row| row.len() == first_row.len()) {
                Some(first_row.len())
            } else {
                None
            }
        })?;
        Some(Self {
            rows: data.len(),
            cols,
            inner: data.into_iter().flatten().collect(),
        })
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no == 0 || row_no > self.rows {
            return None;
        }
        Some(self.inner[self.cols * (row_no - 1)..self.cols * row_no].to_vec())
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no == 0 || col_no > self.cols {
            return None;
        }
        Some(
            (col_no - 1..self.inner.len())
                .step_by(self.cols)
                .map(|index| self.inner[index])
                .collect(),
        )
    }
}
