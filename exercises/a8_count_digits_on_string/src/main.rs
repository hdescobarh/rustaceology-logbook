fn main() {
    let test_list = ["Hello world!", "A2445as5sGAs54%89%", "二"].map(|s| String::from(s));

    for test in test_list {
        println!("\nTest...");
        println!("Text:\n{}\nNumber digits: {}", test, count_digits(&test));
    }
}

fn count_digits(string: &String) -> u128 {
    let counter: u128 = string
        .chars()
        .map(|character| character.is_numeric() as u128)
        .sum();
    counter
}
