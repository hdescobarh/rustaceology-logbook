use std::fmt::{Display, Formatter, Result};

#[derive(Default)]
pub struct Roman {
    digits: Option<Vec<u8>>,
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        todo!("Return a roman-numeral string representation of the Roman object");
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
