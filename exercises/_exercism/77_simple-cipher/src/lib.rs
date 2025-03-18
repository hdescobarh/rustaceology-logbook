pub fn encode(key: &str, plain: &str) -> Option<String> {
    plain
        .chars()
        .zip(key.chars().cycle())
        .map(|(c, shift)| {
            if !shift.is_ascii_lowercase() || !c.is_alphanumeric() {
                return Err(());
            };
            Ok(((c.to_ascii_lowercase() as u8 + shift as u8 - 2 * b'a') % 26 + b'a') as char)
        })
        .collect::<Result<String, ()>>()
        .map(|out| if out.is_empty() { None } else { Some(out) })
        .unwrap_or_default()
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    todo!("Use {key} to decode {s} using shift cipher")
}

pub fn encode_random(s: &str) -> (String, String) {
    todo!("Generate random key with only a-z chars and encode {s}. Return tuple (key, encoded s)")
}
