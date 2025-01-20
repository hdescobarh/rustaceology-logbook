const MINE_BYTE: u8 = 42;
const EMPTY_BYTE: u8 = 32;
const DIGIT_ONE: u8 = 49;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut bytes_table: Vec<Vec<u8>> = minefield
        .iter()
        .map(|row| row.as_bytes().to_owned())
        .collect();

    let field_rows = bytes_table.len();
    let field_cols = bytes_table.first().map(|row| row.len()).unwrap_or(0);

    for row in 0..field_rows {
        for col in 0..field_cols {
            if bytes_table[row][col] != MINE_BYTE {
                continue;
            }

            for (i, j) in get_neighborhood(row, col, field_rows, field_cols) {
                if bytes_table[i][j] == MINE_BYTE {
                    continue;
                } else if bytes_table[i][j] == EMPTY_BYTE {
                    bytes_table[i][j] = DIGIT_ONE
                } else {
                    bytes_table[i][j] += 1
                }
            }
        }
    }

    bytes_table
        .into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .collect()
}

// Gets the valid 1 square neighborhood
fn get_neighborhood(
    row: usize,
    col: usize,
    field_rows: usize,
    field_cols: usize,
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
    .into_iter()
    .filter_map(move |(i, j)| {
        Some((
            row.checked_add_signed(i)
                .and_then(|i| if i < field_rows { Some(i) } else { None })?,
            col.checked_add_signed(j)
                .and_then(|j| if j < field_cols { Some(j) } else { None })?,
        ))
    })
}
