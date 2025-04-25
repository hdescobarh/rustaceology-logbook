use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

/// Type implementing arbitrary-precision decimal arithmetic

#[derive(PartialEq, Eq, Debug)]
pub struct Decimal {
    non_negative: bool,
    point_place: usize,
    /// Digits ordered from the least to the most significant digit
    value: Vec<u8>,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut value: Vec<u8> = Vec::with_capacity(input.len());
        let mut point_place = 0;
        let mut iter = input.bytes().peekable();

        let non_negative = match iter.peek() {
            Some(b'-') => {
                iter.next();
                false
            }
            Some(b'0'..=b'9') => true,
            _ => return None,
        };

        for (index, byte) in iter.rev().enumerate() {
            match byte {
                b'.' if point_place == 0 => point_place = index,
                b'0'..=b'9' => value.push(byte - b'0'),
                _ => return None,
            }
        }
        let mut non_normalized_decimal = Decimal {
            non_negative,
            point_place,
            value,
        };
        non_normalized_decimal.normalize();
        Some(non_normalized_decimal)
    }

    /// Trim non-significant leading and trailing zeros
    fn normalize(&mut self) {
        // remove leading
        let mut leading_zeros = self.value.len();
        for (index, digit) in self.value.iter().rev().enumerate() {
            if *digit != 0 {
                leading_zeros = index;
                break;
            }
        }
        self.value.truncate(self.value.len() - leading_zeros);

        // remove trailing
        let mut trailing_zeros = self.point_place;
        for (index, digit) in self.value[..self.point_place.min(self.value.len())]
            .iter()
            .enumerate()
        {
            if *digit != 0 {
                trailing_zeros = index;
                break;
            }
        }
        self.point_place -= trailing_zeros;
        self.value = self.value[trailing_zeros.min(self.value.len())..].to_vec();
    }

    fn iter_with_padding(&self, trailing: usize, leading: usize) -> PaddedDecimal<'_> {
        PaddedDecimal {
            trailing,
            decimal_value: &self.value,
            leading,
            position: 0,
        }
    }

    fn pairwise<'a>(
        &'a self,
        rhs: &'a Self,
    ) -> impl DoubleEndedIterator + ExactSizeIterator<Item = (&'a u8, &'a u8)> {
        let (self_trailing, rhs_trailing) = match self.point_place.cmp(&rhs.point_place) {
            Ordering::Less => (rhs.point_place - self.point_place, 0),
            Ordering::Equal => (0, 0),
            Ordering::Greater => (0, self.point_place - rhs.point_place),
        };
        let (self_pseudo_len, rhs_pseudo_len) = (
            self_trailing + self.value.len(),
            rhs_trailing + rhs.value.len(),
        );
        let (self_leading, rhs_leading) = match self_pseudo_len.cmp(&rhs_pseudo_len) {
            Ordering::Less => (rhs_pseudo_len - self_pseudo_len, 0),
            Ordering::Equal => (0, 0),
            Ordering::Greater => (0, self_pseudo_len - rhs_pseudo_len),
        };
        self.iter_with_padding(self_trailing, self_leading)
            .zip(rhs.iter_with_padding(rhs_trailing, rhs_leading))
    }

    /// Returns the digit and carry after op
    fn apply<F>(a: u8, b: u8, op: F) -> (u8, u8)
    where
        F: Fn(u8, u8) -> u8,
    {
        let result = (op)(a, b);
        (result % 10, result / 10)
    }

    // This is an attempt of implementing a subtraction that does not rely
    // in the Ordering trait. Returns a value and non_negative  bare fields result
    fn sign_agnostic_sub(&self, rhs: &Self) -> (Vec<u8>, bool) {
        let mut borrow = 0;
        let exact_size_iter = self.pairwise(rhs);
        let mut result = Vec::with_capacity(exact_size_iter.len() + 1);
        for (a, b) in exact_size_iter {
            let to_reduce = b + borrow;
            if *a < to_reduce {
                result.push(10 + a - to_reduce);
                borrow = 1;
            } else {
                result.push(a - to_reduce);
                borrow = 0;
            }
        }
        if borrow == 0 {
            return (result, true);
        };

        let mut carry = 1;
        for digit in result.iter_mut() {
            let raw = 9 - *digit + carry;
            *digit = raw % 10;
            carry = raw / 10;
            if carry == 0 {
                break;
            }
        }
        (result, false)
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

struct PaddedDecimal<'a> {
    trailing: usize,
    decimal_value: &'a [u8],
    leading: usize,
    position: usize,
}

impl<'a> Iterator for PaddedDecimal<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let pseudo_length = self.trailing + self.decimal_value.len() + self.leading;
        if self.position >= pseudo_length {
            return None;
        }
        let item = if self.position < self.trailing {
            &0
        } else if self.position < self.trailing + self.decimal_value.len() {
            &self.decimal_value[self.position - self.trailing]
        } else {
            &0
        };
        self.position += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl DoubleEndedIterator for PaddedDecimal<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let pseudo_length = self.trailing + self.decimal_value.len() + self.leading;
        if self.position >= pseudo_length {
            return None;
        }
        let item = if self.position < self.leading {
            &0
        } else if self.position < self.leading + self.decimal_value.len() {
            &self.decimal_value[self.leading + self.decimal_value.len() - 1 - self.position]
        } else {
            &0
        };
        self.position += 1;
        Some(item)
    }
}

impl ExactSizeIterator for PaddedDecimal<'_> {
    fn len(&self) -> usize {
        self.trailing + self.decimal_value.len() + self.leading - self.position
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let reverse_ord = match self.non_negative.cmp(&rhs.non_negative) {
            Ordering::Equal if self.non_negative => false,
            Ordering::Equal => true,
            ordering => return ordering,
        };
        for (self_item, rhs_item) in self.pairwise(rhs).rev() {
            match self_item.cmp(rhs_item) {
                Ordering::Equal => (),
                ordering if reverse_ord => return ordering.reverse(),
                ordering => return ordering,
            }
        }
        Ordering::Equal
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// input, expected Decimal.value field as &str
    const POSITIVE_WHOLE_NUMBERS: [(&str, &str); 5] = [
        ("9", "9"),
        ("42", "24"),
        ("1602", "2061"),
        ("1457", "7541"),
        ("137035999177", "771999530731"),
    ];

    /// input, expected Decimal.value field as &str, Decimal.point_place field
    const POSITIVE_DECIMAL_GREATER_THAN_ONE: [(&str, &str, usize); 6] = [
        ("9.1", "19", 1),
        ("9.12", "219", 2),
        ("9.001", "1009", 3),
        ("1457.32", "237541", 2),
        ("1.235", "5321", 3),
        ("137.035999139", "931999530731", 9),
    ];

    const POSITIVE_DECIMAL_SMALLER_THAN_ONE: [(&str, &str, usize); 4] = [
        ("0.1", "1", 1),
        ("0.001", "1", 3),
        ("0.123", "321", 3),
        ("0.00123", "321", 5),
    ];

    /// input, expected point place, expected &str
    const POSITIVE_WITH_NON_SIGNIFICANT_ZEROS: [(&str, usize, &str); 10] = [
        ("09", 0, "9"),
        ("009", 0, "9"),
        ("900", 0, "009"),
        ("00900", 0, "009"),
        ("9.00", 0, "9"),
        ("9.100", 1, "19"),
        ("0009.1100", 2, "119"),
        ("0.9", 1, "9"),
        ("0.0900", 2, "9"),
        ("000.0900", 2, "9"),
    ];

    /// input, trailing zeros, leading zeros, expected as &str
    const ITER_WITH_PADDING: [(&str, usize, usize, &str); 7] = [
        ("124", 0, 1, "4210"),
        ("124", 1, 0, "0421"),
        ("124", 2, 3, "00421000"),
        ("124.1475", 1, 2, "0574142100"),
        ("0.0072973525643", 0, 3, "34652537927000"),
        ("0.0072973525643", 2, 0, "0034652537927"),
        ("0.0072973525643", 2, 3, "0034652537927000"),
    ];

    /// Bigger input first
    const SORTED_PAIRS: [[&str; 2]; 21] = [
        // Different sign
        ["1", "-1"],
        ["5.1", "-5.1"],
        ["0", "-1"],
        ["42", "-5"],
        // Same decimal, different whole part
        ["9.1", "0.1"],
        ["2.3", "1.3"],
        ["320.1", "1.1"],
        // Same whole, different decimal part
        ["0.2", "0.1"],
        ["0.2", "0.01"],
        ["0.02", "0.01"],
        ["0.02", "0.0001"],
        ["1.02", "1.01"],
        ["1.02", "1.001"],
        ["1.2", "1.1"],
        ["21.02", "21.01"],
        ["21.2", "21.1"],
        ["21.2", "21.001"],
        // Different whole and decimal parts
        ["5.32", "2.53794"],
        ["5.02", "2.53794"],
        ["5.3218", "2.54"],
        ["5.3218", "0.54"],
    ];

    #[test]
    fn returns_none_with_wrong_input() {
        let cases = [
            "a137.0359",
            "137.0359a",
            "137a0359",
            "137.0359-",
            "137-0359",
            "--1370359",
            "--137.0359",
            "137.0-359",
            "137.-0359",
            "137-.0359",
            "13-7.0359",
            "13.7.0359",
            "137..0359",
        ];
        for input in cases {
            assert_eq!(
                Decimal::try_from(input),
                None,
                "This must fail. Input: {}",
                input
            )
        }
    }

    #[test]
    fn valid_positive_whole_number() {
        for (input, expected_str) in POSITIVE_WHOLE_NUMBERS {
            let expected = Decimal {
                non_negative: true,
                point_place: 0,
                value: expected_str.bytes().map(|b| b - b'0').collect(),
            };
            let actual = Decimal::try_from(input);
            assert_eq!(Some(expected), actual, "Input: {}", input)
        }
    }

    #[test]
    fn valid_negative_whole_number() {
        for (input, expected_str) in POSITIVE_WHOLE_NUMBERS {
            let expected = Decimal {
                non_negative: false,
                point_place: 0,
                value: expected_str.bytes().map(|b| b - b'0').collect(),
            };
            let actual = Decimal::try_from(&format!("-{}", input));
            assert_eq!(Some(expected), actual, "Input: {}", input)
        }
    }

    #[test]
    fn valid_positive_decimal_greater_than_one() {
        for (input, expected_str, point_place) in POSITIVE_DECIMAL_GREATER_THAN_ONE {
            let expected = Decimal {
                non_negative: true,
                point_place,
                value: expected_str.bytes().map(|b| b - b'0').collect(),
            };
            let actual = Decimal::try_from(input);
            assert_eq!(Some(expected), actual, "Input: {}", input)
        }
    }

    #[test]
    fn valid_negative_decimal_greater_than_one() {
        for (input, expected_str, point_place) in POSITIVE_DECIMAL_GREATER_THAN_ONE {
            let expected = Decimal {
                non_negative: false,
                point_place,
                value: expected_str.bytes().map(|b| b - b'0').collect(),
            };
            let actual = Decimal::try_from(&format!("-{}", input));
            assert_eq!(Some(expected), actual, "Input: {}", input)
        }
    }

    #[test]
    fn remove_non_significant_zeros() {
        for (input, point_place, value_str) in POSITIVE_WITH_NON_SIGNIFICANT_ZEROS {
            let expected = Decimal {
                non_negative: true,
                point_place,
                value: value_str.bytes().map(|b| b - b'0').collect(),
            };

            assert_eq!(
                expected,
                Decimal::try_from(input).unwrap(),
                "Used input: {}",
                input
            );
        }
    }

    #[test]
    fn valid_zeros() {
        let cases = ["0", "0000", "0.0000", "0.0", "00000.0000"];
        let expect = Decimal {
            non_negative: true,
            point_place: 0,
            value: vec![],
        };
        for input in cases {
            assert_eq!(
                expect,
                Decimal::try_from(input).unwrap(),
                "Used input: {}",
                input
            )
        }
    }

    #[test]
    fn valid_positive_decimal_smaller_than_one() {
        for (input, expected_str, point_place) in POSITIVE_DECIMAL_SMALLER_THAN_ONE {
            let expected = Decimal {
                non_negative: true,
                point_place,
                value: expected_str.bytes().map(|b| b - b'0').collect(),
            };
            let actual = Decimal::try_from(input);
            assert_eq!(Some(expected), actual, "Input: {}", input)
        }
    }

    #[test]
    fn valid_negative_decimal_smaller_than_one() {
        for (input, expected_str, point_place) in POSITIVE_DECIMAL_SMALLER_THAN_ONE {
            let expected = Decimal {
                non_negative: false,
                point_place,
                value: expected_str.bytes().map(|b| b - b'0').collect(),
            };
            let actual = Decimal::try_from(&format!("-{}", input));
            assert_eq!(Some(expected), actual, "Input: {}", input)
        }
    }

    #[test]
    fn compare_decimals_correctly() {
        let cases = [
            ("0.1", 1, vec![1]),
            ("0.001", 3, vec![1]),
            ("1.1", 1, vec![1, 1]),
            ("1.001", 3, vec![1, 0, 0, 1]),
        ];

        for (input, point_place, value) in cases {
            let expect = Decimal {
                non_negative: true,
                point_place,
                value,
            };
            assert_eq!(Decimal::try_from(input).unwrap(), expect)
        }
    }

    #[test]
    fn returns_padded_decimal() {
        for (input, trailing, leading, expect_str) in ITER_WITH_PADDING {
            let expect: Vec<u8> = expect_str.bytes().map(|b| b - b'0').collect();
            let decimal = Decimal::try_from(input).unwrap();
            let actual: Vec<u8> = decimal
                .iter_with_padding(trailing, leading)
                .cloned()
                .collect();
            assert_eq!(expect, actual)
        }
    }

    #[test]
    fn returns_padded_decimal_reverse() {
        for (input, trailing, leading, expect_str) in ITER_WITH_PADDING {
            let expect: Vec<u8> = expect_str.bytes().rev().map(|b| b - b'0').collect();
            let decimal = Decimal::try_from(input).unwrap();
            let actual: Vec<u8> = decimal
                .iter_with_padding(trailing, leading)
                .rev()
                .cloned()
                .collect();
            assert_eq!(expect, actual)
        }
    }

    #[test]
    fn identify_equal_decimals() {
        for input in SORTED_PAIRS.into_iter().flatten() {
            let decimal_self = Decimal::try_from(input).unwrap();
            let decimal_other = Decimal::try_from(input).unwrap();
            assert!(decimal_self == decimal_other)
        }
    }

    fn additive_inverse(decimal: &Decimal) -> Decimal {
        Decimal {
            non_negative: !decimal.non_negative,
            point_place: decimal.point_place,
            value: decimal.value.clone(),
        }
    }

    #[test]
    fn identify_self_greater_than_other() {
        let assert_cmp = |decimal_self: &Decimal, decimal_other: &Decimal| {
            assert!(
                decimal_self > decimal_other,
                "self: {:?}, other: {:?}",
                decimal_self,
                decimal_other
            )
        };
        for [input_self, input_other] in SORTED_PAIRS {
            let decimal_self = Decimal::try_from(input_self).unwrap();
            let decimal_other = Decimal::try_from(input_other).unwrap();
            assert_cmp(&decimal_self, &decimal_other);
            let (decimal_self, decimal_other) = (
                additive_inverse(&decimal_other),
                additive_inverse(&decimal_self),
            );
            assert_cmp(&decimal_self, &decimal_other);
        }
    }

    #[test]
    fn identify_self_less_than_other() {
        let assert_cmp = |decimal_self: &Decimal, decimal_other: &Decimal| {
            assert!(
                decimal_self < decimal_other,
                "self: {:?}, other: {:?}",
                decimal_self,
                decimal_other
            )
        };
        for [input_other, input_self] in SORTED_PAIRS {
            let decimal_self = Decimal::try_from(input_self).unwrap();
            let decimal_other = Decimal::try_from(input_other).unwrap();
            assert_cmp(&decimal_self, &decimal_other);
            let (decimal_self, decimal_other) = (
                additive_inverse(&decimal_other),
                additive_inverse(&decimal_self),
            );
            assert_cmp(&decimal_self, &decimal_other);
        }
    }
}
