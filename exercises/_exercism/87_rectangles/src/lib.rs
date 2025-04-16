use std::collections::{HashMap, HashSet};
const VERTEX: u8 = b'+';
const H_LINE: u8 = b'-';
const V_LINE: u8 = b'|';
const EMPTY: u8 = b' ';

pub fn count(lines: &[&str]) -> u32 {
    ParsedInput::new(lines)
        .map(|parsed| parsed.count_rectangles())
        .unwrap_or_default()
}

struct ParsedInput {
    /// Maps proper horizontal edges (start_col, end_col) to the rows where they are
    hor_edges_to_rows: HashMap<(usize, usize), Vec<usize>>,
    /// Contains the coordinates of every vertical lines
    ver_edges_coordinates: HashSet<(usize, usize)>,
}

impl ParsedInput {
    fn new(lines: &[&str]) -> Option<Self> {
        lines.first()?;
        let mut hor_edges = HashMap::new();
        let mut ver_edges = HashSet::new();
        for (row, line) in lines.iter().enumerate() {
            let mut segment_start: Option<Vec<usize>> = None;
            for (col, value) in line.bytes().enumerate() {
                match value {
                    VERTEX if segment_start.is_none() => {
                        segment_start = Some(vec![col]);
                        ver_edges.insert((row, col));
                    }
                    VERTEX => {
                        for start in segment_start.as_ref().unwrap() {
                            hor_edges
                                .entry((*start, col))
                                .and_modify(|v: &mut Vec<usize>| v.push(row))
                                .or_insert(vec![row]);
                        }
                        ver_edges.insert((row, col));
                        segment_start.as_mut().unwrap().push(col);
                    }
                    EMPTY => segment_start = None,
                    V_LINE => {
                        segment_start = None;
                        ver_edges.insert((row, col));
                    }
                    H_LINE => continue,
                    _ => return None,
                }
            }
        }
        Some(Self {
            hor_edges_to_rows: hor_edges,
            ver_edges_coordinates: ver_edges,
        })
    }

    /// generate all the possible pair of rows
    fn permutate_rows(rows: &[usize]) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::with_capacity(rows.len() * (rows.len() - 1) / 2);
        for i in 0..rows.len() - 1 {
            for i_prime in i + 1..rows.len() {
                result.push((rows[i], rows[i_prime]))
            }
        }
        result
    }

    fn count_rectangles(&self) -> u32 {
        let mut counter = 0;
        for (hor_segment, rows) in &self.hor_edges_to_rows {
            for candidate in Self::permutate_rows(rows) {
                if (candidate.0 + 1..candidate.1).all(|row| {
                    self.ver_edges_coordinates.contains(&(row, hor_segment.0))
                        && self.ver_edges_coordinates.contains(&(row, hor_segment.1))
                }) {
                    counter += 1;
                };
            }
        }
        counter
    }
}
