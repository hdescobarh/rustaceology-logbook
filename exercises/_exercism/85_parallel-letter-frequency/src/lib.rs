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
    let chunk_size = text.len().div_ceil(worker_count);
    let mut breakpoints: Vec<usize> = vec![0];
    let mut current = 0;
    for _ in 1..worker_count {
        while !text.is_char_boundary(current) {
            current += 1;
        }
        breakpoints.push(current);
        current += chunk_size;
        continue;
    }
    breakpoints.push(text.len() - 1);
    breakpoints
}
