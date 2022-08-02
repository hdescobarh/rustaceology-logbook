use std::io;

fn main() {
    let original_string = read_input();
    let reverse_string = reverse_from_string(&original_string);
    println!(
        "Original:\n{}\nReverse:\n{}",
        original_string, reverse_string
    );
}

fn read_input() -> String {
    let mut input = String::new();
    println!("Write a text:\n");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line.\n");
    input.trim().to_string()
}

fn reverse_from_string(original_string: &String) -> String {
    let lenght = original_string.len();
    let mut char_vector: Vec<char> = vec!['\0'; lenght];
    let mut reverse = String::with_capacity(lenght);
    let mut char_iterator = original_string.chars();

    for index in 0..lenght {
        if let Some(character) = char_iterator.next() {
            char_vector[lenght - 1 - index] = character;
        };
    }

    for character in char_vector {
        reverse.push(character);
    }

    reverse
}
