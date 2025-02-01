use std::collections::HashMap;

use itertools::Itertools;

type Word = Vec<char>;
type Guess = HashMap<char, u8>;

#[cfg_attr(test, derive(Debug, PartialEq))]
struct WordAddition {
    addends: Vec<Word>,
    total: Word,
    alphabet: Word,
    non_zeros: u8,
}

impl WordAddition {
    pub fn new(input: &str) -> Option<Self> {
        let mut parts: Vec<Word> = input
            .split(|c: char| !c.is_ascii_alphabetic())
            .filter(|w| !w.is_empty())
            .map(|w| w.chars().collect())
            .collect();
        let non_zero_letters: Word = parts
            .iter()
            .filter_map(|w| w.first().cloned())
            .unique()
            .collect();
        let alphabet: Word = non_zero_letters
            .iter()
            .chain(parts.iter().flatten())
            .unique()
            .cloned()
            .collect();

        let total = parts.pop()?;

        if parts.is_empty() || alphabet.is_empty() {
            return None;
        }

        let non_zeros = if non_zero_letters.len() < 2 || non_zero_letters.len() > 9 {
            return None;
        } else {
            non_zero_letters.len() as u8
        };

        Some(Self {
            addends: parts,
            total,
            alphabet,
            non_zeros,
        })
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    todo!("Solve the alphametic {input:?}")
}

#[cfg(test)]
mod test {
    use crate::WordAddition;

    #[test]
    fn constructs_correctly_wordaddition() {
        let cases = [
            (
                "I + BB == ILL",
                WordAddition {
                    addends: vec![vec!['I'], vec!['B', 'B']],
                    total: vec!['I', 'L', 'L'],
                    alphabet: vec!['I', 'B', 'L'],
                    non_zeros: 2,
                },
            ),
            (
                "A == B",
                WordAddition {
                    addends: vec![vec!['A']],
                    total: vec!['B'],
                    alphabet: vec!['A', 'B'],
                    non_zeros: 2,
                },
            ),
            (
                "NO + NO + TOO == LATE",
                WordAddition {
                    addends: vec![vec!['N', 'O'], vec!['N', 'O'], vec!['T', 'O', 'O']],
                    total: vec!['L', 'A', 'T', 'E'],
                    alphabet: vec!['N', 'T', 'L', 'O', 'A', 'E'],
                    non_zeros: 3,
                },
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(WordAddition::new(input), Some(expected))
        }
    }
}
