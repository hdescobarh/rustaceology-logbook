fn multiple_conditions() {
    if false {
        println!("This will never print");
    } else if false {
        println!("This neither");
    } else {
        println!("Always This will be printed");
    }
}

fn if_with_assingment() {
    let this_assingment = if true { 42 } else { 0 };
    println!(
        "This remembers me the ternary operators in Java: {}",
        this_assingment
    )
}

fn main() {
    multiple_conditions();
    if_with_assingment();
}
