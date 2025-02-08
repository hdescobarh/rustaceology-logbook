use std::collections::HashSet;
pub fn is_pangram(sentence: &str) -> bool {
    let mut letters: HashSet<char> = HashSet::from_iter('\u{0041}'..='\u{005A}');
    sentence.chars().for_each(|c| {
        letters.remove(&c.to_ascii_uppercase());
    });
    letters.is_empty()
}
