fn main() {
    let input_string = String::from("Muchos años después, frente al pelotón...");
    let output_count = count_characters(&input_string);

    println!("Input:\n{}\nCount:\n{:?}\n", input_string, output_count)
}

// case sensitive
fn count_characters(string: &String) -> Vec<(char, u128)> {
    let mut char_values: Vec<char> = Vec::new();
    let mut count_values: Vec<u128> = Vec::new();
    let mut result: Vec<(char, u128)> = Vec::new();

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
