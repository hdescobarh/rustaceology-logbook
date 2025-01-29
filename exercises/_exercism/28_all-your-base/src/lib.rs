#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    } else if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    Ok(base_from_integer_value(
        get_integer_value(number, from_base)?,
        to_base,
    ))
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

fn base_from_integer_value(mut value: u128, to_base: u32) -> Vec<u32> {
    let mut result = Vec::new();
    while value > 0 {
        result.push((value % to_base as u128) as u32);
        value /= to_base as u128
    }
    result.reverse();
    if result.is_empty() {
        vec![0]
    } else {
        result
    }
}
