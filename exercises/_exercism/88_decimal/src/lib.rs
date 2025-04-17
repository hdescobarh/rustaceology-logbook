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
                b'-' => non_negative = false,
                b'.' => point_place = index,
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

#[cfg(test)]
mod test {
    use super::*;

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
}
