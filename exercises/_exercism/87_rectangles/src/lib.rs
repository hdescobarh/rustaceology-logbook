use std::collections::HashMap;
const EDGE: u8 = b'+';
type segment = (usize, usize);

pub fn count(lines: &[&str]) -> u32 {
    todo!(
        "\nDetermine the count of rectangles in the ASCII diagram represented by the following lines:\n{lines:#?}\n."
    );
}

fn count_segment_overlaps(lines: &[&str]) -> HashMap<segment, usize> {
    let mut result: HashMap<segment, usize> = HashMap::new();
    for line in lines {
        let col_coordinates: Vec<usize> = line
            .bytes()
            .enumerate()
            .filter_map(|(col, byte)| if byte == EDGE { Some(col) } else { None })
            .collect();
        for start in 0..col_coordinates.len() - 1 {
            for end in start + 1..col_coordinates.len() {
                *result.entry((start, end)).or_default() += 1;
            }
        }
    }
    result
}
