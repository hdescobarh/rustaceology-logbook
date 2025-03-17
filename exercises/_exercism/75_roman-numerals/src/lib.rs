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
        let ch = match position {
            0 => ['\0', '\0', 'M'], // ⟸ constructor already ensures that num < 4000
            1 => ['M', 'D', 'C'],
            2 => ['C', 'L', 'X'],
            3 => ['X', 'V', 'I'],
            _ => panic!(),
        };
        match digit {
            0 => "".to_string(),
            1..=3 => ch[2].to_string().repeat(digit.into()),
            4 => format!("{}{}", ch[2], ch[1]),
            5 => format!("{}", ch[1]),
            6..=8 => format!("{}{}", ch[1], ch[2].to_string().repeat((digit - 5).into()),),
            9 => format!("{}{}", ch[2], ch[0]),
            _ => panic!(),
        }
    }
}
