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
        if value < 10 {
            return true;
        }
        let digits: Vec<char> = format!("{}", value).chars().collect();
        for index in 0..digits.len() / 2 {
            if digits[index] != digits[digits.len() - index - 1] {
                return false;
            }
        }
        true
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

#[cfg(test)]
mod test {
    use crate::Palindrome;

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
}
