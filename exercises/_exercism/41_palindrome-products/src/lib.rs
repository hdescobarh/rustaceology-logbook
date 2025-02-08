use std::collections::BTreeMap;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(candidate: (u64, Vec<(u64, u64)>)) -> Self {
        Self {
            value: candidate.0,
            factors: HashSet::from_iter(candidate.1),
        }
    }
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }

    fn is_palindrome(value: u64) -> bool {
        let mut reverse = value;
        let mut result = 0;
        while reverse > 0 {
            result = result * 10 + reverse % 10;
            reverse /= 10;
        }
        result == value
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut candidates: BTreeMap<u64, Vec<(u64, u64)>> = BTreeMap::new();
    (min..=max)
        .flat_map(|j| (min..=j).map(move |i| (i * j, (i, j))))
        .filter(|(product, _)| Palindrome::is_palindrome(*product))
        .for_each(|(product, factors)| {
            candidates
                .entry(product)
                .and_modify(|f| f.push(factors))
                .or_insert(vec![factors]);
        });

    let min_palindrome = Palindrome::new(candidates.pop_first()?);
    let max_palindrome = candidates
        .pop_last()
        .map(Palindrome::new)
        .unwrap_or(min_palindrome.clone());

    Some((min_palindrome, max_palindrome))
}

struct PalindromeOperator {}

impl PalindromeOperator {
    pub fn create(from: u64) -> u64 {
        let mut new = Self::next_palindrome(from, false);
        while new < from {
            new = Self::next_palindrome(new, true)
        }
        new
    }

    pub fn next(from: u64) -> u64 {
        Self::next_palindrome(from, true)
    }

    fn count_digits(mut number: u64) -> u64 {
        if number < 10 {
            return 1;
        }
        let mut count = 0;
        while number > 0 {
            count += 1;
            number /= 10
        }
        count
    }

    fn append_reverse(prefix: u64, mut forward_suffix: u64) -> u64 {
        let mut result = prefix;
        while forward_suffix > 0 {
            let a = forward_suffix % 10;
            result = result * 10 + a;
            forward_suffix /= 10;
        }
        result
    }

    fn next_palindrome(number: u64, from_palindrome: bool) -> u64 {
        let digits = Self::count_digits(number);
        let half_digits = (digits / 2) as u32;
        let prefix = number / 10_u64.pow(half_digits) + if from_palindrome { 1 } else { 0 };
        if digits % 2 == 0 {
            if Self::count_digits(prefix) > half_digits as u64 {
                Self::append_reverse(prefix, prefix / 10)
            } else {
                Self::append_reverse(prefix, prefix)
            }
        } else if Self::count_digits(prefix) > (half_digits + 1) as u64 {
            Self::append_reverse(prefix / 10, prefix / 10)
        } else {
            Self::append_reverse(prefix, prefix / 10)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Palindrome, PalindromeOperator};

    #[test]
    fn check_is_palindrome() {
        let cases = [
            (0, true),
            (9, true),
            (18, false),
            (121, true),
            (9009, true),
            (123321, true),
            (1234321, true),
            (321321, false),
            (3214321, false),
        ];

        for (input, expected) in cases {
            assert_eq!(Palindrome::is_palindrome(input), expected)
        }
    }

    #[test]
    fn next_palindrome_from_odd_to_same_length() {
        let cases = [
            (0, 1),
            (1, 2),
            (101, 111),
            (5_000_005, 5_001_005),
            (5_981_895, 5_982_895),
        ];
        for (input, expected) in cases {
            assert_eq!(PalindromeOperator::next_palindrome(input, true), expected)
        }
    }

    #[test]
    fn next_palindrome_from_odd_to_higher_length() {
        let cases = [
            (9, 11),
            (999, 1001),
            (99_999, 100_001),
            (99_999_999_999, 100_000_000_001),
        ];
        for (input, expected) in cases {
            assert_eq!(PalindromeOperator::next_palindrome(input, true), expected)
        }
    }

    #[test]
    fn next_palindrome_from_even_to_same_length() {
        let cases = [
            (11, 22),
            (99, 101),
            (100_001, 101_101),
            (101_101, 102_201),
            (109_901, 110_011),
            (76_543_211_234_567, 76_543_222_234_567),
        ];
        for (input, expected) in cases {
            assert_eq!(PalindromeOperator::next_palindrome(input, true), expected)
        }
    }

    #[test]
    fn next_palindrome_from_even_to_higher_length() {
        let cases = [
            (9999, 10_001),
            (999_999, 1_000_001),
            (999_999_999_999, 1_000_000_000_001),
        ];
        for (input, expected) in cases {
            assert_eq!(PalindromeOperator::next_palindrome(input, true), expected)
        }
    }
}
