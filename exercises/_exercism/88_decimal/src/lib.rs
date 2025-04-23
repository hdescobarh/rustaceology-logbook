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
        let mut iter = input
            .bytes()
            .rev()
            .skip_while(|v| *v == b'0')
            .enumerate()
            .peekable();
        while let Some((index, byte)) = iter.next() {
            match byte {
                b'-' if iter.peek().is_none() => non_negative = false,
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

    fn as_additive_inverse(&self) -> Self {
        Self {
            non_negative: !self.non_negative,
            point_place: self.point_place,
            value: self.value.clone(),
        }
    }

    fn iter_with_padding(&self, trailing: usize, leading: usize) -> PaddedDecimal<'_> {
        PaddedDecimal {
            trailing,
            decimal_value: &self.value,
            leading,
            position: 0,
        }
    }

    pub fn pairwise<'a>(
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
    /// Perform comparisons in the following order: sign, integer part size,
    /// integer part values, decimal part values (up to the shortest length), and decimal part size.
    /// The comparison may end early if a difference is detected.
    fn cmp(&self, other: &Self) -> Ordering {
        // Sign
        match self.non_negative.cmp(&other.non_negative) {
            Ordering::Equal if self.non_negative => (),
            Ordering::Equal => {
                return other.as_additive_inverse().cmp(&self.as_additive_inverse());
            }
            ordering => return ordering,
        }
        // Integer part
        match (self.value.len() as i128 - self.point_place as i128)
            .cmp(&(other.value.len() as i128 - other.point_place as i128))
        {
            Ordering::Equal => (),
            ordering => return ordering,
        }

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
        // Decimal part
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

    /// input, expected point place, expected &str
    const POSITIVE_WITH_NON_SIGNIFICANT_ZEROS: [(&str, usize, &str); 7] = [
        ("09", 0, "9"),
        ("009", 0, "9"),
        ("900", 0, "009"),
        ("00900", 0, "009"),
        ("9.00", 0, "9"),
        ("9.100", 1, "19"),
        ("0009.1100", 2, "119"),
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
        for (input, point_place, value_str) in POSITIVE_WITH_NON_SIGNIFICANT_ZEROS {
            let expected = Decimal {
                non_negative: true,
                point_place,
                value: value_str.bytes().map(|b| b - b'0').collect(),
            };

            assert_eq!(expected, Decimal::try_from(input).unwrap());
        }
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
    fn negative_decimals() {
        let cases = [
            ("-0.1", 1, vec![1]),
            ("-0.001", 3, vec![1]),
            ("-1.1", 1, vec![1, 1]),
            ("-1.001", 3, vec![1, 0, 0, 1]),
            ("-1.20", 1, vec![2, 1]),
            ("-1.11", 2, vec![1, 1, 1]),
            ("-0.999", 3, vec![9, 9, 9]),
        ];

        for (input, point_place, value) in cases {
            let expect = Decimal {
                non_negative: false,
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
            ("120.1", "1.1"),
            ("5.1", "0.1"),
            // Check decimal part values
            ("0.02", "0.01"),
            ("0.2", "0.1"),
            ("1.02", "1.01"),
            ("1.2", "1.1"),
            ("21.02", "21.01"),
            ("21.2", "21.1"),
            // Check decimal part size
            ("1.22", "1.20"),
            ("1.11", "1.1"),
            ("0.999", "0.99"),
        ];
        let negatives: Vec<(String, String)> = cases_first_greater
            .iter()
            .skip(3)
            .map(|(a, b)| (["-", b].concat(), ["-", a].concat()))
            .collect();
        for (big, small) in cases_first_greater
            .into_iter()
            .chain(negatives.iter().map(|(a, b)| (a.as_str(), b.as_str())))
        {
            let decimal_big = Decimal::try_from(big).unwrap();
            let decimal_small = Decimal::try_from(small).unwrap();
            let expect = Ordering::Greater;
            assert_eq!(
                decimal_big.cmp(&decimal_small),
                expect,
                "\nbig: {decimal_big:?}\n\
                small: {decimal_small:?}\n\
                expect: {expect:?}\n"
            );
            let expect = Ordering::Less;
            assert_eq!(
                decimal_small.cmp(&decimal_big),
                expect,
                "\nbig: {decimal_big:?}\n\
                small: {decimal_small:?}\n\
                expect: {expect:?}\n"
            );
            for decimal in [decimal_big, decimal_small] {
                let expect = Ordering::Equal;
                assert_eq!(
                    decimal.cmp(&decimal),
                    expect,
                    "\ndecimal: {decimal:?}\n\
                    expect: {expect:?}\n"
                );
            }
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
}
