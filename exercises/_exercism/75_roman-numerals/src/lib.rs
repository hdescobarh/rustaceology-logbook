use std::fmt::{Display, Formatter, Result};

#[derive(Default)]
pub struct Roman {
    digits: Option<Vec<u8>>,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(v) = &self.digits {
            v.iter().enumerate().try_for_each(|(position, &digit)| {
                write!(f, "{}", Self::digit_to_roman(position, digit))
            })?;
        };
        Ok(())
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let mut roman: Roman = Default::default();
        if num == 0 || num > 3999 {
            return roman;
        }
        let mut digits = Vec::with_capacity(4);
        for divisor in [1000, 100, 10, 1] {
            digits.push((num / divisor) as u8);
            num %= divisor;
        }
        roman.digits = Some(digits);
        roman
    }
}

impl Roman {
    fn digit_to_roman(position: usize, digit: u8) -> String {
        let chars = match position {
            3 => ['\0', '\0', 'M'], // ⟸ constructor already ensures that num < 4000
            2 => ['M', 'D', 'C'],
            1 => ['C', 'L', 'X'],
            0 => ['X', 'V', 'I'],
            _ => panic!(),
        };
        match digit {
            0..4 => chars[2].to_string().repeat(digit.into()),
            4 => format!("{}{}", chars[2], chars[1]),
            5 => format!("{}", chars[1]),
            6..9 => format!("{}{}", chars[1], chars[2].to_string().repeat(digit.into()),),
            9 => format!("{}{}", chars[2], chars[0]),
            _ => panic!(),
        }
    }
}
