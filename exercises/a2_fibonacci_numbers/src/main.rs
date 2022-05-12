use std::io;

fn input_position() -> u32 {
    println!("Which nth number do you want to find?:");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    input.trim().parse().expect("This is not a number")
}

fn raw_fibonacci(position: u32) -> u32 {
    let mut counter = 0;
    let mut current_a: u32 = 0;
    let mut current_b: u32 = 1;

    if position == 0 {
        return 0;
    }

    while counter < position {
        let intermediate = current_a;
        current_a = current_b;
        current_b = current_b + intermediate;
        counter = counter + 1;
    }

    return current_a;
}

fn naive_recursive_fibonacci(position: u32) -> u32 {
    let base_case: [u32; 2] = [0, 1];

    if base_case.contains(&position) {
        return position;
    } else {
        return naive_recursive_fibonacci(position - 1) + naive_recursive_fibonacci(position - 2);
    }
}

fn main() {
    let position = input_position();

    let result_raw = raw_fibonacci(position);
    println!("Direct fibo: F_{}= {}", position, result_raw);
    let result_recursive = naive_recursive_fibonacci(position);
    println!("Naive recursive: F_{}= {}", position, result_recursive);
}
