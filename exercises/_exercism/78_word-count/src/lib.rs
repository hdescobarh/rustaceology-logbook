use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(text: &str) -> HashMap<String, u32> {
    let mut output = HashMap::new();
    for word in text.split(|c: char| !c.is_ascii_alphanumeric() && c != '\'') {
        if let Some(parsed_word) = parse_word(word) {
            output
                .entry(parsed_word)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    output
}

fn parse_word(word: &str) -> Option<String> {
    let last_index = word.len().saturating_sub(1); // Only ASCII characters are allowed; bytes and character counts are equal.
    let out: String = word
        .char_indices()
        .filter_map(|(index, c)| {
            if (index == 0 || index == last_index) && c == '\'' {
                None
            } else {
                Some(c.to_ascii_lowercase())
            }
        })
        .collect();
    if out.is_empty() { None } else { Some(out) }
}
