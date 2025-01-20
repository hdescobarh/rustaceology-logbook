const MINE_BYTE: u8 = 42;
const EMPTY_BYTE: u8 = 32;
const DIGIT_ONE: u8 = 49;
const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut board: Vec<Vec<u8>> = minefield
        .iter()
        .map(|row| row.as_bytes().to_owned())
        .collect();

    let height = board.len();
    let width = board.first().map(|row| row.len()).unwrap_or(0);

    for row in 0..height {
        for col in 0..width {
            if board[row][col] != MINE_BYTE {
                continue;
            }

            get_neighborhood(row, col, height, width).for_each(|(i, j)| {
                if board[i][j] == EMPTY_BYTE {
                    board[i][j] = DIGIT_ONE
                } else if board[i][j] != MINE_BYTE {
                    board[i][j] += 1
                }
            })
        }
    }

    board
        .into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .collect()
}

// Gets the valid 1 square neighborhood
fn get_neighborhood(
    row: usize,
    col: usize,
    height: usize,
    width: usize,
) -> impl Iterator<Item = (usize, usize)> {
    NEIGHBORS
        .iter()
        .filter_map(move |(i, j)| Some((row.checked_add_signed(*i)?, col.checked_add_signed(*j)?)))
        .filter(move |(i, j)| *i < height && *j < width)
}
