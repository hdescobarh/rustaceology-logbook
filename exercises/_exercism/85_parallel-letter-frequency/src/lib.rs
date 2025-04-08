use std::collections::HashMap;
use std::thread;

const PARALLEL_THRESHOLD: usize = 50;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    match (input.len(), worker_count) {
        (_, 0) => panic!("worker_count must be greater than zero."),
        (0, _) => return HashMap::new(),
        (size, _) if size < PARALLEL_THRESHOLD => return letter_counter(input),
        _ => (),
    }

    thread::scope(|scope| {
        let mut handles = Vec::with_capacity(worker_count);
        for chunk in input.chunks(input.len().div_ceil(worker_count)) {
            let handle = scope.spawn(move || letter_counter(chunk));
            handles.push(handle);
        }
        let mut result = match handles.pop() {
            Some(handle) => handle.join().unwrap(),
            None => HashMap::new(),
        };
        for handle in handles {
            for (letter, count) in handle.join().unwrap() {
                *result.entry(letter).or_default() += count;
            }
        }
        result
    })
}
fn letter_counter(text: &[&str]) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for letter in text.iter().flat_map(|line| line.chars()) {
        if !letter.is_alphabetic() {
            continue;
        }
        if let Some(letter) = letter.to_lowercase().next() {
            *result.entry(letter).or_default() += 1;
        }
    }
    result
}
