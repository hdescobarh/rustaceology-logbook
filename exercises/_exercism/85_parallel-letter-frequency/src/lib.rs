use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    match input.len() {
        0 => return HashMap::new(),
        v if v < 100 => return count(input),
        _ => (),
    };
    thread::scope(|scope| {
        let mut result = HashMap::new();
        let mut handles = vec![];
        for chunk in input.chunks(input.len().div_ceil(worker_count)) {
            let handle = scope.spawn(move || count(chunk));
            handles.push(handle);
        }
        for handle in handles {
            for (letter, count) in handle.join().unwrap() {
                *result.entry(letter).or_default() += count;
            }
        }
        result
    })
}
fn count(text: &[&str]) -> HashMap<char, usize> {
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
