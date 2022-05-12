use std::io;

fn option_input() -> String {
    println!(
        "Choose and option:\n
    1 Celsius to Fahrenheit\n
    2 Fahrenheit to Celsius\n"
    );

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn option_manager() -> String {
    let valid_input: [String; 2] = ["1".to_string(), "2".to_string()];

    let mut chosen_option = option_input();

    while !valid_input.contains(&chosen_option) {
        println!("Please make a valid choice\n");
        chosen_option = option_input();
    }

    chosen_option
}

fn temperature_input() -> i32 {
    let mut input = String::new();
    println!("Please insert the temperature:\n");
    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading the input");

    // I did not read the error handling chapter yet
    input.trim().parse().expect("This is not a number")
}

fn fharenheit_to_celsius(temperature_celsius: i32) -> i32 {
    (temperature_celsius - 32) * 5 / 9
}

fn celsius_to_fharenheit(temperature_fharenheit: i32) -> i32 {
    (temperature_fharenheit * 9 / 5) + 32
}

fn main() {
    let option = option_manager();

    if option == "1" {
        println!("{}°F", celsius_to_fharenheit(temperature_input()))
    } else if option == "2" {
        println!("{}°C", fharenheit_to_celsius(temperature_input()))
    } else {
        println!("Unexpected error in options")
    }
}
