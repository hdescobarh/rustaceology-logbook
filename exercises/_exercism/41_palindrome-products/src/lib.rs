use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    number: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(number: u64, factors: HashSet<(u64, u64)>) -> Self {
        Self { number, factors }
    }
    pub fn value(&self) -> u64 {
        self.number
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let smallest = PalinOps::find_smallest_pal(min, max)?;
    let highest = PalinOps::find_largest_pal(max, &smallest);
    Some((smallest, highest))
}

struct PalinOps {}

impl PalinOps {
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
    // given prefix (dn..d0) and suffix: (am..a0), generates (dn..d0 a0..am)
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

    pub fn find_smallest_pal(min: u64, max: u64) -> Option<Palindrome> {
        let mut candidate = PalinOps::create(min.pow(2));
        let mut factors = HashSet::new();
        while candidate <= max.pow(2) {
            for a in min..=candidate.isqrt() {
                if candidate % a == 0 {
                    factors.insert((a, candidate / a));
                }
            }
            if !factors.is_empty() {
                break;
            }
            candidate = PalinOps::next(candidate);
        }
        if factors.is_empty() {
            None
        } else {
            Some(Palindrome::new(candidate, factors))
        }
    }

    pub fn find_largest_pal(max: u64, smallest: &Palindrome) -> Palindrome {
        let (mut largest_pal, mut largest_factors) = (smallest.number, smallest.factors.clone());
        let (mut pal, mut factors) = (PalinOps::next(largest_pal), HashSet::new());
        while pal <= max.pow(2) {
            for a in pal.isqrt()..=max {
                if pal % a == 0 {
                    factors.insert((pal / a, a));
                }
            }
            if !factors.is_empty() {
                (largest_pal, largest_factors, factors) = (pal, factors, HashSet::new());
            }
            pal = PalinOps::next(pal);
        }

        Palindrome::new(largest_pal, largest_factors)
    }
}

#[cfg(test)]
mod test {
    use crate::PalinOps;

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
            assert_eq!(PalinOps::next_palindrome(input, true), expected)
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
            assert_eq!(PalinOps::next_palindrome(input, true), expected)
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
            assert_eq!(PalinOps::next_palindrome(input, true), expected)
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
            assert_eq!(PalinOps::next_palindrome(input, true), expected)
        }
    }
}
