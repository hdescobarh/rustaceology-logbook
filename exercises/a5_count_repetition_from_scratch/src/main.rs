use std::collections::HashMap;
fn main() {
    // case sensitive - variant sensitive (e.g. half-width ｳ vs full width ウ; Ｄ vs D)

    let input_string =
        String::from("Muchos años ｳウｄdespués,  frente al pelotón...字典¡!사전¿?👾辞書");
    let output_count = count_characters_scratch(&input_string);
    let output_count_2 = count_character(&input_string);

    println!(
        "Input:\n{}\nCount from scratch:\n{:?}\nCount 2:\n{:?}\n",
        input_string, output_count, output_count_2
    )
}

fn count_character(string: &String) -> HashMap<char, usize> {
    let mut unique_char: Vec<char> = string.chars().collect();
    unique_char.sort();
    unique_char.dedup(); // sort + dup -> remove all duplicates
    let mut count_store: HashMap<char, usize> = HashMap::new();

    for character in unique_char {
        count_store.insert(character, string.matches(character).count());
    }

    count_store
}

// Using only tools of prelude
fn count_characters_scratch(string: &String) -> Vec<(char, usize)> {
    let mut char_values: Vec<char> = Vec::new();
    let mut count_values: Vec<usize> = Vec::new();
    let mut result: Vec<(char, usize)> = Vec::new();

    for character in string.chars() {
        // if char in char_values then add one more to the same index in count_values
        // else add the new char and add value 1 to index
        match is_char_on_store(&char_values, &character) {
            Check::IsIn(index) => count_values[index] = count_values[index] + 1,
            Check::IsNotIn => {
                char_values.push(character);
                count_values.push(1)
            }
        }
    }

    for index in 0..char_values.len() {
        result.push((char_values[index], count_values[index]));
    }

    result.sort();
    result
}

fn is_char_on_store(store: &Vec<char>, character: &char) -> Check {
    let mut index = 0usize;

    for value in store {
        if character == value {
            return Check::IsIn(index);
        }
        index = index + 1;
    }

    return Check::IsNotIn;
}

enum Check {
    IsIn(usize),
    IsNotIn,
}
