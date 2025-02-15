// Given a matrix mxn the spiral movement follows this pattern (steps direction):
// n →, m-1 ↓, n-1 ←,  m-2 ↑, n-2 →, ... continues decreasing the number of steps
// in each direction by 1 in each cycle, until all the nm cells have been visited.
// This means, the sequence can end in two ways:
// (a) ..., m-k, n-k; k = m-1 if m <= n
// (b) ..., n-(k-1), m-k; k = n if m > n
// The total number of steps is given by:
// f(k) = n + \sum_{i=1}^k (m-i) + \sum_{i=1}^k (n-i) =
// n + km + kn - k(k+1) = n + k (m+n-k-1)
// Replacing k why should get mn:
// (a) f(m-1) = mn
// (b) f(n) = mn
// Then, the strategy is to implement a generator of the pattern
#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Init,
    Right(usize),
    Down(usize),
    Left(usize),
    Up(usize),
}

impl Direction {
    fn next(&self, steps: usize) -> Self {
        match self {
            Direction::Init => Direction::Right(steps),
            Direction::Right(_) => Direction::Down(steps),
            Direction::Down(_) => Direction::Left(steps),
            Direction::Left(_) => Direction::Up(steps),
            Direction::Up(_) => Direction::Right(steps),
        }
    }
}
struct Movement {
    remaining_rows: usize,
    remaining_cols: usize,
    cycle: usize,
    direction: Direction,
}

impl Movement {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            remaining_rows: rows - 1,
            remaining_cols: cols,
            cycle: 0,
            direction: Direction::Init,
        }
    }
}

impl Iterator for Movement {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycle % 2 == 0 {
            self.direction = self.direction.next(self.remaining_cols);
            self.remaining_cols = self.remaining_cols.checked_sub(1)?;
        } else {
            self.direction = self.direction.next(self.remaining_rows);
            self.remaining_rows = self.remaining_rows.checked_sub(1)?;
        }
        self.cycle += 1;
        Some(self.direction)
    }
}
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    todo!("Function that returns the spiral matrix of square size {size}");
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn directions_cols_smallest() {
        let (rows, cols) = (3, 4);
        let moves: Vec<Direction> = Movement::new(rows, cols).take(rows * cols).collect();
        let expect = vec![
            Direction::Right(4),
            Direction::Down(2),
            Direction::Left(3),
            Direction::Up(1),
            Direction::Right(2),
        ];
        assert_eq!(moves, expect)
    }

    #[test]
    fn directions_rows_smallest() {
        let (rows, cols) = (4, 3);
        let moves: Vec<Direction> = Movement::new(rows, cols).take(rows * cols).collect();
        let expect = vec![
            Direction::Right(3),
            Direction::Down(3),
            Direction::Left(2),
            Direction::Up(2),
            Direction::Right(1),
            Direction::Down(1),
        ];
        assert_eq!(moves, expect)
    }
}
