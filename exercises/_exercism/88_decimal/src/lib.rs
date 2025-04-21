use std::cmp::Ordering;

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
        let mut non_negative = true;
        for (index, byte) in input.bytes().rev().skip_while(|v| *v == b'0').enumerate() {
            match byte {
                b'-' if index == input.len() - 1 => non_negative = false,
                b'.' if point_place == 0 => point_place = index,
                b'0'..=b'9' => value.push(byte - b'0'),
                _ => return None,
            }
        }
        let mut decimal = Self {
            non_negative,
            point_place,
            value,
        };
        decimal.trim_leading_zeros();
        Some(decimal)
    }
    fn trim_leading_zeros(&mut self) {
        let mut leading_zeros = self.value.len();
        for (index, v) in self.value.iter().rev().enumerate() {
            if *v != 0 {
                leading_zeros = index;
                break;
            }
        }
        self.value.truncate(self.value.len() - leading_zeros);
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    /// Perform comparisons in the following order: sign, integer part size,
    /// integer part values, decimal part values (up to the shortest length), and decimal part size.
    /// The comparison may end early if a difference is detected.
    fn cmp(&self, other: &Self) -> Ordering {
        // Integer part
        match self.non_negative.cmp(&other.non_negative) {
            Ordering::Equal => (),
            ordering => return ordering,
        }

        match (self.value.len() as i128 - self.point_place as i128)
            .cmp(&(other.value.len() as i128 - other.point_place as i128))
        {
            Ordering::Equal => (),
            ordering => return ordering,
        }
        // Decimal part
        match (
            self.value.get(self.point_place..),
            other.value.get(other.point_place..),
        ) {
            (None, None) => (),
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(self_items), Some(other_items)) => {
                for (self_v, other_v) in self_items.iter().rev().zip(other_items.iter().rev()) {
                    match self_v.cmp(other_v) {
                        Ordering::Equal => (),
                        ordering => return ordering,
                    }
                }
            }
        };

        for (self_v, other_v) in self.value[..self.point_place.min(self.value.len())]
            .iter()
            .rev()
            .zip(
                other.value[..other.point_place.min(other.value.len())]
                    .iter()
                    .rev(),
            )
        {
            match self_v.cmp(other_v) {
                Ordering::Equal => (),
                ordering => return ordering,
            }
        }

        match self.point_place.cmp(&other.point_place) {
            Ordering::Equal => (),
            ordering => return ordering,
        }

        Ordering::Equal
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_none_with_wrong_input() {
        let cases = [
            "a137.0359",
            "137.0359a",
            "137a0359",
            "137.0359-",
            "137-0359",
            "137.0-359",
            "137.-0359",
            "137-.0359",
            "13-7.0359",
            "13.7.0359",
            "137..0359",
        ];
        for input in cases {
            assert_eq!(Decimal::try_from(input), None)
        }
    }

    #[test]
    fn valid_positive_int_number() {
        let input = "1002";
        let expect = Decimal {
            non_negative: true,
            point_place: 0,
            value: vec![2, 0, 0, 1],
        };
        assert_eq!(Decimal::try_from(input).unwrap(), expect)
    }

    #[test]
    fn valid_negative_int_number() {
        let input = "-1002";
        let expect = Decimal {
            non_negative: false,
            point_place: 0,
            value: vec![2, 0, 0, 1],
        };
        assert_eq!(Decimal::try_from(input).unwrap(), expect)
    }

    #[test]
    fn valid_positive_float_number() {
        let input = "1.002";
        let expect = Decimal {
            non_negative: true,
            point_place: 3,
            value: vec![2, 0, 0, 1],
        };
        assert_eq!(Decimal::try_from(input).unwrap(), expect)
    }

    #[test]
    fn valid_negative_float_number() {
        let input = "-1.002";
        let expect = Decimal {
            non_negative: false,
            point_place: 3,
            value: vec![2, 0, 0, 1],
        };
        assert_eq!(Decimal::try_from(input).unwrap(), expect)
    }

    #[test]
    fn remove_non_significant_zeros() {
        let input = "001.02000";
        let expect = Decimal {
            non_negative: true,
            point_place: 2,
            value: vec![2, 0, 1],
        };
        assert_eq!(Decimal::try_from(input).unwrap(), expect)
    }

    #[test]
    fn valid_zeros() {
        let cases = ["0", "0.0", "0000", "0.0000", "00000.0000"];
        let expect = Decimal {
            non_negative: true,
            point_place: 0,
            value: vec![],
        };
        for input in cases {
            assert_eq!(Decimal::try_from(input).unwrap(), expect)
        }
    }

    #[test]
    fn non_negative_decimals() {
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
    fn order_non_equal_decimals_correctly() {
        let cases_first_greater = [
            // Check sign
            ("1", "-1"),
            ("5.1", "-5.1"),
            ("0", "-1"),
            // Check integer part size
            ("21.1", "0.1"),
            // Check integer part values
            ("11.1", "10.1"),
            // Check decimal part size
            ("1.2", "1.12"),
            ("1.11", "1.1"),
            // Check decimal part values
            ("0.02", "0.01"),
            ("0.2", "0.1"),
            ("1.02", "1.01"),
            ("1.2", "1.1"),
            ("21.02", "21.01"),
            ("21.2", "21.1"),
        ];
        for (big, small) in cases_first_greater {
            let decimal_big = Decimal::try_from(big).unwrap();
            let decimal_small = Decimal::try_from(small).unwrap();
            let expect = Ordering::Greater;
            assert_eq!(
                decimal_big.cmp(&decimal_small),
                expect,
                "\nbig: {decimal_big:?}\n\
                small: {decimal_small:?}\n\
                expect: {expect:?}\n"
            )
        }
    }
}
