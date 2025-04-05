use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut output = HashMap::new();
    for thread_counts in multi_thread_count(&input.concat(), worker_count) {
        for (letter, frequency) in thread_counts {
            output
                .entry(letter)
                .and_modify(|old| *old += frequency)
                .or_insert(frequency);
        }
    }
    output
}

// The idea is to have chunks but without allocation overhead
fn get_breakpoints(text: &str, worker_count: usize) -> Vec<usize> {
    let chunk_size = (text.len() / worker_count).max(1);
    let mut breakpoints = vec![0];
    let mut current = chunk_size;
    while breakpoints.len() <= worker_count && current < text.len() {
        while !text.is_char_boundary(current) && current < text.len() {
            current += 1;
        }
        breakpoints.push(current);
        current = (current + chunk_size).min(text.len())
    }
    if *breakpoints.last().unwrap() != text.len() {
        breakpoints.push(text.len())
    };
    breakpoints
}

fn single_thread_count(text: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for letter in text.chars() {
        if letter.is_alphabetic() {
            counts
                .entry(letter.to_lowercase().next().unwrap())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    counts
}

fn multi_thread_count(text: &str, worker_count: usize) -> Vec<HashMap<char, usize>> {
    if text.is_empty() {
        return vec![];
    }
    let breakpoints = get_breakpoints(text, worker_count);
    thread::scope(|scope| {
        let mut handles = Vec::with_capacity(worker_count);
        for pair in breakpoints.windows(2) {
            let (start, end) = (pair[0], pair[1]);
            handles.push(scope.spawn(move || single_thread_count(&text[start..end])));
        }
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect()
    })
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn empty_text() {
        let text = "";
        let expect = [0];
        let cases = [1, 4, 10];
        for worker_count in cases {
            assert_eq!(get_breakpoints(text, worker_count), expect)
        }
    }

    #[test]
    fn ascii_text_length_one() {
        let text = "a";
        let expect = [0, 1];
        let cases = [1, 4, 10];
        for worker_count in cases {
            assert_eq!(get_breakpoints(text, worker_count), expect)
        }
    }

    #[test]
    fn ascii_text_large_than_one() {
        let text = "ab cd ef gh ";
        let cases = [
            (1, vec![0, 12]),
            (4, vec![0, 3, 6, 9, 12]),
            (6, vec![0, 2, 4, 6, 8, 10, 12]),
            (10, (0..11).chain(12..13).collect()),
            (11, (0..13).collect()),
            (12, (0..13).collect()),
            (15, (0..13).collect()),
        ];
        for (worker_count, expect) in cases {
            assert_eq!(get_breakpoints(text, worker_count), expect)
        }
    }

    #[test]
    fn unicode_text_length_one() {
        let cases = [
            ("A", [0, 1]),  // one byte,
            ("à", [0, 2]),  // two bytes
            ("は", [0, 3]), //three bytes
            ("👾", [0, 4]), //four bytes
        ];
        for worker_count in [1, 2, 3, 4, 7] {
            for (text, expect) in cases {
                assert_eq!(get_breakpoints(text, worker_count), expect)
            }
        }
    }

    #[test]
    fn unicode_text_large_than_one() {
        let text = "áàは갎智ghЁ"; // 17 bytes, 8 chars at (0,2,4,7,10,13,14,15)
        let cases = [
            (1, vec![0, 17]),
            (2, vec![0, 10, 17]),
            (3, vec![0, 7, 13, 17]),
            (7, vec![0, 2, 4, 7, 10, 13, 15, 17]),
            (8, vec![0, 2, 4, 7, 10, 13, 15, 17]),
            (9, vec![0, 2, 4, 7, 10, 13, 14, 15, 17]),
            (15, vec![0, 2, 4, 7, 10, 13, 14, 15, 17]),
            (17, vec![0, 2, 4, 7, 10, 13, 14, 15, 17]),
            (20, vec![0, 2, 4, 7, 10, 13, 14, 15, 17]),
        ];
        for (worker_count, expect) in cases {
            assert_eq!(get_breakpoints(text, worker_count), expect)
        }
    }
}
