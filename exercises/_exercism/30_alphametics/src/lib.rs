use std::collections::HashMap;

use itertools::{Itertools, Position};

type Word = Vec<char>;
type Guess = HashMap<char, u8>;

#[cfg_attr(test, derive(Debug, PartialEq))]
struct WordAddition {
    addends: Vec<Word>,
    total: Word,
    alphabet: Word,
    non_zeros: usize,
    powers: Vec<i128>,
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

        let mut powers_sum: HashMap<char, i128> = HashMap::new();
        for (position, word) in parts.iter().with_position() {
            let operation = match position {
                Position::Last => -1,
                _ => 1,
            };
            for (exponent, &letter) in word.iter().rev().enumerate() {
                let value = operation * 10_i128.pow(u32::try_from(exponent).ok()?);
                let current = powers_sum
                    .entry(letter)
                    .and_modify(|v| *v += value)
                    .or_insert(value);
            }
        }

        let powers = alphabet
            .iter()
            .filter_map(|c| powers_sum.get(c))
            .cloned()
            .collect();

        let total = parts.pop()?;

        if parts.is_empty() || alphabet.is_empty() {
            return None;
        }

        let non_zeros = if non_zero_letters.len() < 2 || non_zero_letters.len() > 9 {
            return None;
        } else {
            non_zero_letters.len()
        };

        Some(Self {
            addends: parts,
            total,
            alphabet,
            non_zeros,
            powers,
        })
    }

    pub fn brute_force_solve(&self) -> Option<Guess> {
        for perm in (0_u8..=9).permutations(self.alphabet.len()) {
            if perm.get(0..self.non_zeros)?.contains(&0) {
                continue;
            }
            if self
                .powers
                .iter()
                .zip(perm.iter())
                .fold(0, |total, (coefficient, digit)| {
                    total + coefficient * *digit as i128
                })
                == 0
            {
                return Some(
                    self.alphabet
                        .iter()
                        .cloned()
                        .zip(perm)
                        .collect::<HashMap<char, u8>>(),
                );
            }
        }

        None
    }
}

pub fn solve(input: &str) -> Option<Guess> {
    WordAddition::new(input)?.brute_force_solve()
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
                    powers: vec![-99, 11, -11],
                },
            ),
            (
                "A == B",
                WordAddition {
                    addends: vec![vec!['A']],
                    total: vec!['B'],
                    alphabet: vec!['A', 'B'],
                    non_zeros: 2,
                    powers: vec![1, -1],
                },
            ),
            (
                "NO + NO + TOO == LATE",
                WordAddition {
                    addends: vec![vec!['N', 'O'], vec!['N', 'O'], vec!['T', 'O', 'O']],
                    total: vec!['L', 'A', 'T', 'E'],
                    alphabet: vec!['N', 'T', 'L', 'O', 'A', 'E'],
                    non_zeros: 3,
                    powers: vec![20, 90, -1000, 13, -100, -1],
                },
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(WordAddition::new(input), Some(expected))
        }
    }
}
