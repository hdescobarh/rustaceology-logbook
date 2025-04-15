use std::collections::HashMap;
const EDGE: u8 = b'+';
type Segment = (usize, usize);

pub fn count(lines: &[&str]) -> u32 {
    count_segment_overlaps(lines)
        .into_values()
        .map(|count| count * (count - 1) / 2)
        .sum()
}

fn count_segment_overlaps(lines: &[&str]) -> HashMap<Segment, u32> {
    let mut result: HashMap<Segment, u32> = HashMap::new();
    for line in lines {
        let col_coordinates: Vec<usize> = line
            .bytes()
            .enumerate()
            .filter_map(|(col, byte)| if byte == EDGE { Some(col) } else { None })
            .collect();
        if col_coordinates.len() < 2 {
            continue;
        }
        for start in 0..col_coordinates.len() - 1 {
            for end in start + 1..col_coordinates.len() {
                *result
                    .entry((col_coordinates[start], col_coordinates[end]))
                    .or_default() += 1;
            }
        }
    }
    result
}
