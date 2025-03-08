pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        self.to_string()
            .chars()
            .filter_map(|c| {
                if c.is_ascii_whitespace() {
                    None
                } else {
                    Some(c.to_digit(10).ok_or(()))
                }
            })
            .rev()
            .try_fold((0, false, 0), |(sum, double, size), maybe_digit| {
                maybe_digit.map(|digit| {
                    let sum = if double {
                        sum + if digit < 5 { digit * 2 } else { digit * 2 - 9 }
                    } else {
                        sum + digit
                    };
                    (sum, !double, size + 1)
                })
            })
            .is_ok_and(|(sum, _, size)| sum.rem_euclid(10) == 0 && size > 1)
    }
}
