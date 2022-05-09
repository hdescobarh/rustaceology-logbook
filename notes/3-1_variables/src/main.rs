fn main() {
    // the default behaviour of variables
    let inmutable_var: i32 = 3;
    println!("The original value of inmutable_var is: {}", inmutable_var);

    // shadowing a variable means to declare a new variable with the same name
    let inmutable_var = 3.1416;
    println!(
        "After shadowing, the new value inmutable_var is: {}, note that now it is float",
        inmutable_var
    );

    // how to make variables mutable
    let mut mutable_var = 5;
    println!("The initial value of mutable_var is: {}", mutable_var);
    mutable_var = 6;
    println!(
        "Now the value of mutable_var is (type cannot be changed): {}",
        mutable_var
    );

    // how are CONSTANTS
    const THIS_IS_A_CONSTANT: i32 = 0;
    println!("The value of THIS_IS_A_CONSTANT is: {}", THIS_IS_A_CONSTANT);
}
