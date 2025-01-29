#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if let Some(&d) = number.iter().find(|d| **d >= from_base) {
        return Err(Error::InvalidDigit(d));
    }

    if [0, 1].contains(&from_base) {
        return Err(Error::InvalidInputBase);
    }

    if [0, 1].contains(&to_base) {
        return Err(Error::InvalidOutputBase);
    }

    let result = base_from_integer_value(get_integer_value(number, from_base), to_base);

    if result.is_empty() {
        Ok(vec![0])
    } else {
        Ok(result)
    }
}

fn get_integer_value(number: &[u32], from_base: u32) -> u128 {
    let mut result = 0;
    for (position, &value) in number.iter().rev().enumerate() {
        result += value as u128 * (from_base as u128).pow(position as u32)
    }
    result
}

fn base_from_integer_value(mut value: u128, to_base: u32) -> Vec<u32> {
    let mut result = Vec::new();
    while value >= to_base as u128 {
        result.push((value % to_base as u128) as u32);
        value /= to_base as u128
    }
    if value != 0 {
        result.push(value as u32)
    }
    result.reverse();
    result
}
