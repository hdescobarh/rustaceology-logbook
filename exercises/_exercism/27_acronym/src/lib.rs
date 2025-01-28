pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&['-', ' '])
        .map(|word| {
            if word.chars().any(|c| c.is_alphabetic() && c.is_lowercase()) {
                initial_from_mixed(word)
            } else {
                intial_from_all_upper(word)
            }
        })
        .collect()
}

fn initial_from_mixed(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphabetic())
        .enumerate()
        .filter_map(|(index, c)| {
            if index == 0 || c.is_uppercase() {
                Some(c)
            } else {
                None
            }
        })
        .collect::<String>()
        .to_uppercase()
}

fn intial_from_all_upper(word: &str) -> String {
    match word.chars().find(|c| c.is_uppercase()) {
        Some(c) => c.to_string(),
        None => "".to_string(),
    }
}
