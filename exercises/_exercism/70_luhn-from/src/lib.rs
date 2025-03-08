pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.check_chars_rev()
            .try_fold((0, false, 0), |mut acc, maybe_digit| {
                maybe_digit.map(|digit| {
                    if acc.1 {
                        acc.0 += if digit < 5 { digit * 2 } else { digit * 2 - 9 };
                    } else {
                        acc.0 += digit
                    }
                    acc.1 = !acc.1;
                    acc.2 += 1;
                    acc
                })
            })
            .is_ok_and(|(sum, _, length)| sum.rem_euclid(10) == 0 && length > 1)
    }

    fn check_chars_rev(&self) -> impl Iterator<Item = Result<u32, ()>> {
        self.0
            .chars()
            .filter_map(|c| {
                if c.is_ascii_whitespace() {
                    None
                } else {
                    Some(c.to_digit(10).ok_or(()))
                }
            })
            .rev()
    }
}

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}
