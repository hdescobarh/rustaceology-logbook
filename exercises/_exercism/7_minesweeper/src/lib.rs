const MINE_BYTE: u8 = 42;
const EMPTY_BYTE: u8 = 32;
const DIGIT_ONE: u8 = 49;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut board: Vec<Vec<u8>> = minefield
        .iter()
        .map(|row| row.as_bytes().to_owned())
        .collect();

    let width = board.len();
    let height = board.first().map(|row| row.len()).unwrap_or(0);

    for row in 0..width {
        for col in 0..height {
            if board[row][col] != MINE_BYTE {
                continue;
            }

            for (i, j) in get_neighborhood(row, col, width, height) {
                if board[i][j] == MINE_BYTE {
                    continue;
                } else if board[i][j] == EMPTY_BYTE {
                    board[i][j] = DIGIT_ONE
                } else {
                    board[i][j] += 1
                }
            }
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
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter_map(move |(i, j)| {
        Some((
            row.checked_add_signed(*i)
                .and_then(|i| if i < width { Some(i) } else { None })?,
            col.checked_add_signed(*j)
                .and_then(|j| if j < height { Some(j) } else { None })?,
        ))
    })
}
