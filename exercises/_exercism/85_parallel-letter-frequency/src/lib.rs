use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    todo!(
        "Count the frequency of letters in the given input '{input:?}'. Ensure that you are using {} to process the input.",
        match worker_count {
            1 => "1 worker".to_string(),
            _ => format!("{worker_count} workers"),
        }
    );
}

fn get_breakpoints(text: &str, worker_count: usize) -> Vec<usize> {
    let chunk_size = (text.len() / worker_count).max(1);
    let mut breakpoints: Vec<usize> = Vec::with_capacity(worker_count.min(text.len()));
    let mut current = 0;
    for _ in 0..breakpoints.capacity() {
        while !text.is_char_boundary(current) && current < text.len() - 1 {
            current += 1;
        }
        breakpoints.push(current);
        current += chunk_size;
    }
    if let Some(true) = breakpoints.last().map(|v| *v < text.len() - 1) {
        breakpoints.push(text.len() - 1);
    }
    breakpoints
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn ascii_non_empty_text() {
        let text = "ab cd ef gh ";
        let cases = [
            (4, vec![0, 3, 6, 9, 11]),
            (6, vec![0, 2, 4, 6, 8, 10, 11]),
            (12, (0..12).collect()),
            (15, (0..12).collect()),
        ];
        for (worker_count, expect) in cases {
            assert_eq!(get_breakpoints(text, worker_count), expect)
        }
    }
}
