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

pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .flat_map(AtbashCipher::cypher_to_plain)
        .collect()
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
        let decoded = match letter {
            'a'..='z' => (2 * b'a' + 25 - letter as u8).into(),
            '0'..='9' => letter,
            _ => return None,
        };
        Some(decoded)
    }
}
