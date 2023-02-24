// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
use core::panic;

#[derive(PartialEq, Debug)]
pub enum KindOfChar {
    Vowel(char, String),
    Consonant(char, String),
    Other(String),
}

pub struct PigLatinTraductor {
    original_text: String,
    piglatin_text: String,
}

impl PigLatinTraductor {
    pub fn new(original_text: &str) -> Self {
        // Text must not empty
        if original_text.is_empty() {
            panic!("The text is empty. Must introduce a non-empty text")
        }
        // Text must have only ASCII compatible values
        for character in original_text.chars() {
            if !character.is_ascii() {
                panic!("Only accepts values from Basic Latin Unicode block (U+0000..U+007F)");
            }
        }
        // Text must have at least one alphanumeric character
        if !original_text
            .chars()
            .any(|character| character.is_alphanumeric())
        {
            panic!("Text must have at least one alphanumeric character");
        };

        Self {
            original_text: original_text.to_string(),
            piglatin_text: String::new(),
        }
    }

    fn decompose_text(&self) -> Option<Vec<usize>> {
        let mut char_iterator = self.original_text.char_indices();
        let mut split_reference = match char_iterator.next() {
            Some((_, character)) => character.is_alphabetic(),
            None => return None,
        };
        let mut slices_index: Vec<usize> = vec![0];
        // the final value of this must be the number of characters + 1
        let mut last_index = 1usize;

        for (index, character) in char_iterator {
            last_index += 1;
            if character.is_alphabetic() != split_reference {
                slices_index.push(index);
                split_reference = !split_reference;
            }
        }
        slices_index.push(last_index);

        return Some(slices_index);
    }

    fn starts_with(word: &str) -> KindOfChar {
        // Discard numbers, control characters, etc.
        if word.starts_with(|c: char| !c.is_alphabetic()) || word.is_empty() {
            return KindOfChar::Other(word.to_string());
        };

        // the word is guaranteed to be of at least size 1
        let mut characters = word.chars();
        let first_char = characters.next().unwrap();
        let remaining_chars = characters.collect();

        if ['a', 'e', 'i', 'o', 'u'].contains(&first_char.to_ascii_lowercase()) {
            KindOfChar::Vowel(first_char, remaining_chars)
        } else {
            KindOfChar::Consonant(first_char, remaining_chars)
        }
    }

    //
    pub fn traduce(&mut self) -> &String {
        let mut new_text = String::new();
        let word_indices = self.decompose_text().unwrap();
        for index in 0..(word_indices.len() - 1) {
            //
            let word = &self.original_text[word_indices[index]..word_indices[index + 1]];

            match PigLatinTraductor::starts_with(word) {
                KindOfChar::Vowel(first_char, remaining_chars) => {
                    new_text.push_str(&format!("{first_char}{remaining_chars}-hay"))
                }
                KindOfChar::Consonant(first_char, remaining_chars) => {
                    if remaining_chars.is_empty() {
                        new_text.push_str(&format!("{first_char}ay"));
                    } else {
                        new_text.push_str(&format!(
                            "{remaining}-{consonant}ay",
                            remaining = remaining_chars,
                            consonant = first_char.to_ascii_lowercase()
                        ));
                    }
                }
                KindOfChar::Other(word) => new_text.push_str(&word),
            }
        }

        if new_text.starts_with(" ") {
            new_text.remove(0);
        }
        self.piglatin_text = new_text;

        return &self.piglatin_text;
    }

    // This is a naive implementation. There are a lot of nested if-else
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "Only accepts values from Basic Latin Unicode block (U+0000..U+007F)"]
    fn fails_to_create_traductor_because_invalid_characters() {
        PigLatinTraductor::new("おはよう😀幸");
    }

    #[test]
    #[should_panic = "The text is empty. Must introduce a non-empty text"]
    fn fails_to_create_traductor_because_empty_text() {
        PigLatinTraductor::new("");
    }

    #[test]
    #[should_panic = "Text must have at least one alphanumeric character"]
    fn fails_to_create_traductor_because_doesnt_contain_alphanumeric_values() {
        PigLatinTraductor::new(" \n \t");
    }

    #[test]
    fn test_word_decomposition_indexing() {
        let a_single_word_have_two_indexes =
            PigLatinTraductor::new("First").decompose_text().unwrap();
        assert_eq!(vec![0, 5], a_single_word_have_two_indexes);

        let starts_alphabetic_ends_nond_alphabetic = PigLatinTraductor::new("Haword\nX.")
            .decompose_text()
            .unwrap();
        assert_eq!(vec![0, 6, 7, 8, 9], starts_alphabetic_ends_nond_alphabetic);

        let starts_non_alphabetic_ends_alphabetic = PigLatinTraductor::new(" aword\nXal")
            .decompose_text()
            .unwrap();
        assert_eq!(vec![0, 1, 6, 7, 10], starts_non_alphabetic_ends_alphabetic);
    }

    #[test]
    fn correctly_indicates_which_character_starts_with() {
        assert_eq!(
            KindOfChar::Consonant('S', String::new()),
            PigLatinTraductor::starts_with("S")
        );

        assert_eq!(
            KindOfChar::Consonant('F', "irst".to_string()),
            PigLatinTraductor::starts_with("First")
        );

        assert_eq!(
            KindOfChar::Vowel('a', String::new()),
            PigLatinTraductor::starts_with("a")
        );

        assert_eq!(
            KindOfChar::Vowel('a', "pple".to_string()),
            PigLatinTraductor::starts_with("apple")
        );

        assert_eq!(
            KindOfChar::Other("\n".to_string()),
            PigLatinTraductor::starts_with("\n")
        );
    }

    #[test]
    fn correctly_traduce_from_vowel_starting_single_word() {
        assert_eq!("a-hay", PigLatinTraductor::new("a").traduce());
        assert_eq!("A-hay", PigLatinTraductor::new("A").traduce());
        assert_eq!("apple-hay", PigLatinTraductor::new("apple").traduce());
        assert_eq!("Apple-hay", PigLatinTraductor::new("Apple").traduce());
    }

    #[test]
    fn correctly_traduce_from_consonant_starting_single_word() {
        assert_eq!("Bay", PigLatinTraductor::new("B").traduce());
        assert_eq!("irst-fay", PigLatinTraductor::new("first").traduce());
        assert_eq!("irst-fay", PigLatinTraductor::new("First").traduce());
    }

    #[test]
    fn correctly_traduce_a_phrase() {
        let phrase_one = "Many years later, as he faced the firing squad, Colonel Aureliano Buendia
        (...).";

        let traduction_uncorrected_capitalization = "any-may ears-yay ater-lay, as-hay e-hay aced-fay he-tay iring-fay quad-say, olonel-cay Aureliano-hay uendia-bay
        (...).";

        assert_eq!(
            traduction_uncorrected_capitalization,
            PigLatinTraductor::new(phrase_one).traduce()
        );
    }
}
