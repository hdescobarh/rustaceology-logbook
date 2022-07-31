fn main() {
    let a_number = Number::Odd(11);

    // Whith match
    match a_number {
        Number::Odd(value) => println!("With match: {}", value.rem_euclid(2) == 1),
        _ => (), // boiler plate, neccesary to ensure exhaustiviness check
    };

    // Whit if let
    if let Number::Odd(value) = a_number {
        println!("Withc if let: {}", value.rem_euclid(2) == 1)
    }
}

enum Number {
    Odd(i128),
    _Even(i128),
}
