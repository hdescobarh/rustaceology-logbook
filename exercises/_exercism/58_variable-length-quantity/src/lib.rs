#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut output: Vec<u8> = values
        .iter()
        .rev()
        .flat_map(|&number| encode_single_value(number))
        .collect();
    output.reverse();
    output
}

fn encode_single_value(mut number: u32) -> impl Iterator<Item = u8> {
    (0..).map_while(move |index| {
        match (index, number) {
            (0, 0) => return Some(0_u8),
            (_, 0) => return None,
            _ => (),
        }
        let octet = (number & 127) as u8; // get 0 followed by the last seven bits
        number >>= 7; // remove those read bits
        if index == 0 {
            Some(octet)
        } else {
            Some(octet | 128)
        }
    })
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result: Vec<u32> = Vec::new();
    let mut number: Vec<u8> = Vec::new();
    for b in bytes {
        let septet = b & 127; // remove the most significant bit
        number.push(septet);
        if septet == *b {
            result.push(decode_single_number(&number));
            number = Vec::new();
        }
    }
    if !number.is_empty() {
        return Err(Error::IncompleteNumber);
    }
    Ok(result)
}

fn decode_single_number(number: &[u8]) -> u32 {
    number
        .iter()
        .fold(0_u32, |acc, septet| (acc << 7) | *septet as u32)
}
