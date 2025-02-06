use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
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
    todo!(
        "returns the minimum and maximum number of palindromes of the products of two factors in the range {min} to {max}"
    );
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
