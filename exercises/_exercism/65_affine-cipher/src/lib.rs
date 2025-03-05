#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(u32),
}

pub fn encode(plaintext: &str, a: u32, b: u32) -> Result<String, AffineCipherError> {
    let cipher = AffineCipher::new(a, b)?;
    todo!("Encode {plaintext} with the key ({a}, {b})");
}

pub fn decode(ciphertext: &str, a: u32, b: u32) -> Result<String, AffineCipherError> {
    let cipher = AffineCipher::new(a, b)?;
    todo!("Decode {ciphertext} with the key ({a}, {b})");
}

struct AffineCipher(u32, u32);

impl AffineCipher {
    pub fn new(a: u32, b: u32) -> Result<Self, AffineCipherError> {
        if Self::greatest_common_divisor(a, b) == 1 {
            return Err(AffineCipherError::NotCoprime(1));
        };
        Ok(AffineCipher(a, b))
    }

    fn plain_to_cypher(&self, letter: char) -> Option<char> {
        let letter_index = match letter {
            'A'..='Z' => (b'A' - letter as u8) as u32,
            'a'..='z' => (b'a' - letter as u8) as u32,
            '0'..='1' => return Some(letter),
            _ => return None,
        };
        let cypher_letter = ((self.0 * letter_index + self.1) % 26) as u8;
        Some(cypher_letter.into())
    }

    fn greatest_common_divisor(a: u32, b: u32) -> u32 {
        let (max, min) = if a >= b { (a, b) } else { (b, a) };
        if min == 0 {
            return max;
        }
        Self::greatest_common_divisor(min, max % min)
    }
}
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn gcd_not_coprime() {
        let cases = [
            (2, 8, 2),
            (3, 27, 3),
            (64, 1024, 64),
            (1071, 462, 21),
            (1236, 2460, 12),
            (5275, 7425, 25),
            (8120, 9240, 280),
        ];
        for (a, b, expected) in cases {
            assert_eq!(AffineCipher::greatest_common_divisor(a, b), expected);
            assert_eq!(AffineCipher::greatest_common_divisor(b, a), expected);
        }
    }

    #[test]
    fn gcd_coprime() {
        let cases = [
            (2, 3, 1),
            (1012, 1013, 1),
            (2021, 3037, 1),
            (4096, 6561, 1),
            (5003, 7001, 1),
            (8125, 9007, 1),
        ];
        for (a, b, expected) in cases {
            assert_eq!(AffineCipher::greatest_common_divisor(a, b), expected);
            assert_eq!(AffineCipher::greatest_common_divisor(b, a), expected);
        }
    }
}
