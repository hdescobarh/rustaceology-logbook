use std::io;
use std::mem::replace;

fn input_position() -> usize {
    println!("Which ith fibonacci number do you want to find?:");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line.\n");

    input
        .trim()
        .parse::<usize>()
        .expect("This is not a number.\n")
}

fn fibonacci_iterative(ith_fibonacci: usize) -> Option<u128> {
    let mut current_fibo_value: u128 = 0;
    let mut later_fibo_value: u128 = 1;
    let mut buffer: u128;

    for _i in 0..ith_fibonacci {
        buffer = replace(&mut later_fibo_value, current_fibo_value);
        let check_overflow = current_fibo_value.overflowing_add(buffer);
        if check_overflow.1 {
            return None;
        } else {
            current_fibo_value = check_overflow.0;
        }
    }

    Some(current_fibo_value)
}

fn fibonacci_recursive(ith_fibonacci: usize) -> u128 {
    if ith_fibonacci <= 1 {
        return u128::try_from(ith_fibonacci).unwrap();
    }

    fibonacci_recursive(ith_fibonacci - 2) + fibonacci_recursive(ith_fibonacci - 1)
}

fn main() {
    let position = input_position();

    let result_raw = fibonacci_iterative(position);
    match result_raw {
        None => println!("Overflow. Value bigger than {}", u128::MAX),
        Some(value) => println!("Iterative: F_'{{'{}'}}'= {}", position, value),
    }

    // To avoid the long running time of recursive algorithm
    if position <= 42 {
        let result_recursive = fibonacci_recursive(position);
        println!("Recursive: F_'{{'{}'}}'= {}", position, result_recursive);
    }
}
