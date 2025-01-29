#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
    Overflow,
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    } else if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }
    base_from_integer_value(get_integer_value(number, from_base)?, to_base)
}

fn get_integer_value(number: &[u32], from_base: u32) -> Result<u128, Error> {
    let mut result = 0;
    for &digit in number.iter() {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        result = result * from_base as u128 + digit as u128
    }
    Ok(result)
}

fn base_from_integer_value(mut value: u128, to_base: u32) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    while value > 0 {
        match u32::try_from(value % to_base as u128) {
            Ok(value) => result.push(value),
            Err(_) => return Err(Error::Overflow),
        };
        value /= to_base as u128
    }
    result.reverse();
    if result.is_empty() {
        Ok(vec![0])
    } else {
        Ok(result)
    }
}
