use std::io;

fn main() {
    // Example
    demo();
    // Input
    let original_string = read_input();
    let reverse_easy = invert_from_string_easy(&original_string);
    let reverse_string_scratch = reverse_from_string_scratch(&original_string);
    println!(
        "Original:\n{}\nReverse simple:\n{}\nReverse implemented from scratch:\n{}\n",
        original_string, reverse_easy, reverse_string_scratch
    );
}

fn demo() {
    println!("Demo start:\n");
    let demo = String::from("Muchos años después, frente al pelotón...字典¡!사전¿?👾辞書");
    let reverted_input = invert_from_string_easy(&demo);
    let reverted_scratch = reverse_from_string_scratch(&demo);
    println!(
        "Original:\n{}\nReverse simple:\n{}\nReverse implemented from scratch:\n{}\n",
        demo, reverted_input, reverted_scratch
    );
    println!("Demo end:\n");
}

fn read_input() -> String {
    let mut input = String::new();
    println!("Write a text:\n");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line.\n");
    input.trim().to_string()
}

fn invert_from_string_easy(original: &String) -> String {
    let char_iterator = original.chars().rev();
    String::from_iter(char_iterator)
}

fn reverse_from_string_scratch(original_string: &String) -> String {
    let lenght = original_string.len();
    let mut char_vector: Vec<char> = vec!['\0'; lenght]; //Generates error if try to acces index i when the actual lenght is lower than i
    let mut reverse = String::with_capacity(lenght);
    let mut char_iterator = original_string.chars(); // Using method nth(i) will remove i
    let max_index = lenght - 1;

    for index in 0..lenght {
        if let Some(character) = char_iterator.next() {
            char_vector[max_index - index] = character;
        };
    }

    for character in char_vector {
        reverse.push(character);
    }

    reverse
}
