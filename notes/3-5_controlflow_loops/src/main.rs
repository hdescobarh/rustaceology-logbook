fn how_loop_works() {
    // Example from The Rust Programming Language

    let mut count = 0;

    //This is a LOOP LABEL
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                // this will break the innermost loop
                break;
            }
            if count == 2 {
                //This will break and go to the labeled loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn conditional_loop() {
    const MAX_ALLOWED_VALUE: i32 = 3;
    let mut current_value = 0;

    while current_value <= MAX_ALLOWED_VALUE {
        println!("{}", current_value);
        current_value = current_value + 1;
    }
}

fn collection_loop() {
    // this is better than a while with index because: 1. It is less prone to human error.
    // 2. It is faster since this do not add conditional checks (wich needs runtime code)

    let this_collection: [f64; 3] = [2.71828, 1.618033, 3.141592];

    for item in this_collection {
        println!("The value is {}", item);
    }
}

fn main() {
    println!("\nHow a loop works:");
    how_loop_works();
    println!("\nConditional loop with while:");
    conditional_loop();
    println!("\nLoop through a collection:");
    collection_loop();
}
