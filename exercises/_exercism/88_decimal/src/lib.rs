/// Type implementing arbitrary-precision decimal arithmetic

#[derive(PartialEq, Eq, Debug)]
pub struct Decimal {
    non_negative: bool,
    point_place: usize,
    value: Vec<u8>,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut value: Vec<u8> = Vec::with_capacity(input.len());
        let mut point_place = 0;
        let mut non_negative = true;
        for (index, byte) in input.bytes().rev().enumerate() {
            match byte {
                b'-' => non_negative = false,
                b'.' => point_place = index,
                b'0'..=b'9' => value.push(byte - b'0'),
                _ => return None,
            }
        }
        Some(Self {
            non_negative,
            point_place,
            value,
        })
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
}
