use std::collections::HashMap;
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

fn fibonacci_recursive_with_memoization(ith_fibonacci: usize) -> Option<u128> {
    let mut memory: HashMap<usize, u128> = HashMap::from([(0, 0), (1, 1)]);

    fn inner_recursion<'a>(
        ith_fibonacci: usize,
        memory: &'a mut HashMap<usize, u128>,
    ) -> Option<&'a u128> {
        if memory.contains_key(&ith_fibonacci) {
            return memory.get(&ith_fibonacci);
        }

        let buffer1 = inner_recursion(ith_fibonacci - 1, memory).unwrap().clone();
        let buffer2 = inner_recursion(ith_fibonacci - 2, memory).unwrap().clone();
        memory.insert(ith_fibonacci, buffer1 + buffer2);
        memory.get(&ith_fibonacci)
    }

    let result = inner_recursion(ith_fibonacci, &mut memory).unwrap();

    if result < &u128::MAX {
        memory.remove(&ith_fibonacci)
    } else {
        None
    }
}

fn main() {
    let position = input_position();

    let result_raw = fibonacci_iterative(position);
    match result_raw {
        None => println!("🚫 Overflow. Value bigger than {}", u128::MAX),
        Some(value) => println!("👌 Iterative: F_'{{'{}'}}'= {}", position, value),
    }

    // To avoid the long running time of recursive algorithm
    if position <= 42 {
        let result_recursive = fibonacci_recursive(position);
        println!("👌 Recursive: F_'{{'{}'}}'= {}", position, result_recursive);
    } else {
        println!("🚫 {} ith is too long for simple recursive", position)
    }

    let result_memoization = fibonacci_recursive_with_memoization(position);
    match result_memoization {
        None => println!("🚫 Overflow. Value bigger than {}", u128::MAX),
        Some(value) => println!(
            "👌 Recursive with memoization: F_'{{'{}'}}'= {}",
            position, value
        ),
    }
}
