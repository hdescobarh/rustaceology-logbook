fn parser<F: Fn(char, char) -> u8>(key: &str, text: &str, op: F) -> Option<String> {
    text.chars()
        .zip(key.chars().cycle())
        .map(|(c, shift)| {
            if !shift.is_ascii_lowercase() || !c.is_alphanumeric() {
                return Err(());
            };
            Ok(((op)(c, shift) % 26 + b'a') as char)
        })
        .collect::<Result<String, ()>>()
        .map(|out| if out.is_empty() { None } else { Some(out) })
        .unwrap_or_default()
}

pub fn encode(key: &str, plain: &str) -> Option<String> {
    parser(key, plain, |c, shift| {
        c.to_ascii_lowercase() as u8 + shift as u8 - 2 * b'a'
    })
}

pub fn decode(key: &str, cipher: &str) -> Option<String> {
    parser(key, cipher, |c, shift| {
        c.to_ascii_lowercase() as u8 + 26 - shift as u8
    })
}

pub fn encode_random(s: &str) -> (String, String) {
    todo!("Generate random key with only a-z chars and encode {s}. Return tuple (key, encoded s)")
}
