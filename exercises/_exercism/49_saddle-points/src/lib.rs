type Coordinate = (usize, usize);
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<Coordinate> {
    let (rows, cols) = if input.is_empty() || input[0].is_empty() {
        return vec![];
    } else {
        (input.len(), input[0].len())
    };
    let tallest_size_by_row: Vec<u64> =
        (0..rows).map(|i| *input[i].iter().max().unwrap()).collect();
    let shortest_size_by_col: Vec<u64> = (0..cols)
        .map(|j| input.iter().map(|row| row[j]).min().unwrap())
        .collect();
    let output: Vec<Coordinate> = (0..rows)
        .flat_map(|i| (0..cols).map(move |j| (i, j)))
        .filter(|&(i, j)| tallest_size_by_row[i] == shortest_size_by_col[j])
        .collect();

    if output.is_empty() {
        Vec::new()
    } else {
        output
    }
}
