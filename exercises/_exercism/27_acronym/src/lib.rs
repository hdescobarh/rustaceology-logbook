use std::fmt::Write;

pub fn abbreviate(phrase: &str) -> String {
    let mut buffer = String::new();
    for word in phrase.split(&['-', ' ']) {
        let letters = if word.chars().any(|c| c.is_alphabetic() && c.is_lowercase()) {
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
        } else {
            match word.chars().find(|c| c.is_uppercase()) {
                Some(c) => c.to_string(),
                None => "".to_string(),
            }
        };

        write!(&mut buffer, "{}", letters).unwrap();
    }
    buffer
}
