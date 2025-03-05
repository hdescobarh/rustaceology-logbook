/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .flat_map(AtbashCipher::plain_to_cypher)
        .enumerate()
        .fold(String::new(), |mut acc, (index, letter)| {
            if index % 5 == 0 && index > 0 {
                acc.push(' ');
            }
            acc.push(letter);
            acc
        })
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    todo!("Decoding of {cipher:?} in Atbash cipher.");
}

struct AtbashCipher;

impl AtbashCipher {
    fn plain_to_cypher(letter: char) -> Option<char> {
        let encoded = match letter {
            'A'..='Z' => (b'A' + b'a' + 25 - letter as u8).into(),
            'a'..='z' => (2 * b'a' + 25 - letter as u8).into(),
            '0'..='9' => letter,
            _ => return None,
        };
        Some(encoded)
    }

    fn cypher_to_plain(letter: char) -> Option<char> {
        let letter_index = match letter {
            'a'..='a' => todo!(),
            '0'..='9' => return Some(letter),
            _ => return None,
        };
        todo!()
    }
}
