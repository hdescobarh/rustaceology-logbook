use std::ops::Neg;

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(u8),
}

// Modulo 26 operations: no need to allow negative or large keys,
// as they all reduce to values in 0..=25
pub fn encode(plaintext: &str, a: u8, b: u8) -> Result<String, AffineCipherError> {
    AffineCipher::new(a, b).map(|cipher| {
        plaintext
            .chars()
            .flat_map(|letter| cipher.plain_to_cypher(letter))
            .enumerate()
            .fold(String::new(), |mut acc, (i, letter)| {
                if i > 0 && i % 5 == 0 {
                    acc.push(' ');
                };
                acc.push(letter);
                acc
            })
    })
}

pub fn decode(ciphertext: &str, a: u8, b: u8) -> Result<String, AffineCipherError> {
    AffineCipher::new(a, b).map(|cipher| {
        ciphertext
            .chars()
            .flat_map(|letter| cipher.cypher_to_plain(letter))
            .collect::<String>()
    })
}

struct AffineCipher {
    a: u16,
    b: u16,
    a_mul_inverse: u16,
    b_add_inverse: u16,
}

impl AffineCipher {
    pub fn new(a: u8, b: u8) -> Result<Self, AffineCipherError> {
        let a_inverse = match Self::extended_euclidean(a as i16, 26) {
            (1, inverse, _) => inverse.rem_euclid(26) as u16,
            _ => return Err(AffineCipherError::NotCoprime(a)),
        };
        Ok(Self {
            a: a as u16,
            b: b as u16,
            a_mul_inverse: a_inverse,
            b_add_inverse: (b as i16).neg().rem_euclid(26) as u16,
        })
    }

    fn plain_to_cypher(&self, letter: char) -> Option<char> {
        let letter_index = match letter {
            'A'..='Z' => letter as u16 - b'A' as u16,
            'a'..='z' => letter as u16 - b'a' as u16,
            '0'..='9' => return Some(letter),
            _ => return None,
        };
        let encoded_index = (self.a * letter_index + self.b) % 26;
        Some((b'a' + encoded_index as u8).into())
    }

    fn cypher_to_plain(&self, letter: char) -> Option<char> {
        let letter_index = match letter {
            'a'..='z' => letter as u16 - b'a' as u16,
            '0'..='9' => return Some(letter),
            _ => return None,
        };
        let decoded_index = self.a_mul_inverse * (letter_index + self.b_add_inverse) % 26;
        Some((b'a' + decoded_index as u8).into())
    }

    /// Computes the gcd and the Bézout's identity coefficients
    fn extended_euclidean(a: i16, m: i16) -> (i16, i16, i16) {
        if m == 0 {
            return (a, 1, 0);
        }
        let (gcd, x, y) = Self::extended_euclidean(m, a % m);
        (gcd, y, x - (a / m) * y)
    }
}
