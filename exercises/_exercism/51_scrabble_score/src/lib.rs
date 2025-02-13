const ALPHABET_SCORES: [u64; 26] = [
    1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
];

pub fn score(word: &str) -> u64 {
    word.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .fold(0, |acc, ascii_alphabetic| {
            let byte = ascii_alphabetic as u8;
            let index = if (65..=90).contains(&byte) {
                byte - 65
            } else {
                byte - 97
            };
            acc + ALPHABET_SCORES[index as usize]
        })
}
