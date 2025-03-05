#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(u8),
}

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
    let cipher = AffineCipher::new(a, b)?;
    todo!("Decode {ciphertext} with the key ({a}, {b})");
}

struct AffineCipher(u16, u16);

impl AffineCipher {
    pub fn new(a: u8, b: u8) -> Result<Self, AffineCipherError> {
        let (max, min) = if a >= 26 { (a, 26) } else { (26, a) };
        match Self::extended_euclidean(max as i16, min as i16) {
            (1, _, _) => (),
            _ => return Err(AffineCipherError::NotCoprime(a)),
        }
        Ok(AffineCipher(a as u16, b as u16))
    }

    fn plain_to_cypher(&self, letter: char) -> Option<char> {
        let letter_index = match letter {
            'A'..='Z' => letter as u8 - b'A',
            'a'..='z' => letter as u8 - b'a',
            '0'..='9' => return Some(letter),
            _ => return None,
        };
        let cypher_letter = b'a' + ((self.0 * letter_index as u16 + self.1) % 26) as u8;
        Some(cypher_letter.into())
    }

    /// Computes the gcd and the Bézout's identity coefficients
    fn extended_euclidean(a: i16, b: i16) -> (i16, i16, i16) {
        if b == 0 {
            return (a, 1, 0);
        }
        let (gcd, x, y) = Self::extended_euclidean(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}
