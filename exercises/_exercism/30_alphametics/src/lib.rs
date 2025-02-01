use std::collections::HashMap;

use itertools::Itertools;

type Word = Vec<char>;
type Guess = HashMap<char, u8>;

#[cfg_attr(test, derive(Debug, PartialEq))]
struct WordAddition {
    addends: Vec<Word>,
    total: Word,
    alphabet: Word,
    non_zeros: usize,
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
            non_zero_letters.len()
        };

        Some(Self {
            addends: parts,
            total,
            alphabet,
            non_zeros,
        })
    }

    fn get_integer_value(word: &Word, guess: &Guess) -> Option<u128> {
        let mut total = 0_u128;
        for c in word {
            total = 10 * total + *guess.get(c)? as u128;
        }
        Some(total)
    }

    fn check_guess(&self, guess: &Guess) -> Option<bool> {
        let guessed_total = Self::get_integer_value(&self.total, guess)?;
        let mut guessed_sum = 0_u128;
        for word in &self.addends {
            guessed_sum += Self::get_integer_value(word, guess)?
        }
        if guessed_total == guessed_sum {
            Some(true)
        } else {
            Some(false)
        }
    }

    pub fn brute_force_solve(&self) -> Option<Guess> {
        for perm in (0_u8..=9).permutations(self.alphabet.len()) {
            if perm.get(0..self.non_zeros)?.contains(&0) {
                continue;
            }
            let guess = self
                .alphabet
                .iter()
                .cloned()
                .zip(perm)
                .collect::<HashMap<char, u8>>();

            if let Some(true) = self.check_guess(&guess) {
                return Some(guess);
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
