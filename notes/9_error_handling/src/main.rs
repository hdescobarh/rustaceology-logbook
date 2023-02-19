use std::char::ParseCharError;
use std::str::FromStr;
fn main() {
    // When they are not expected or the error can open a security vulnerability (such as accesing to certains memory addresses)
    unrecoverable_errors();
    // When in it is possible to fail under normal conditions; for example, in a CLI the user makes and invalid input
    let recoverable_assingment = recoverable_errors();
    // Manage errors with match
    matching_errors();
    // Error propagation
    error_propagation_verbose();
    error_propagation_short();
}

fn recoverable_errors() -> Result<char, ParseCharError> {
    let result_type = char::from_str("a");
    return result_type;
}

fn unrecoverable_errors() {
    let a_vector = vec![0, 1, 1, 2, 3];
    // unwind panic will first clean the stack and then quit
    println!("let unwind_panic = a_vector[10]\n// index is bigger than the actual vector size, this will panic");
    // abort will quit inmediatly, the cleaning of the stack must be performed by the OS
    use std::process::abort;
    let abort = match a_vector.get(1) {
        Some(value) => value,
        None => abort(),
    };
}

fn matching_errors() {
    // You can match a Result<T, Err>
    let this_can_panic: char = match char::from_str("a") {
        Ok(value) => value,
        Err(error) => panic!("Parsing from String Failed"),
    };
    // Implementing with Match can be verbose, Standard library have utilities to simplify it; for example: expect implements panic!(msg)
    let this_can_panic = char::from_str("a").expect("Parsing from String Failed");
    // Or this allows assing a default value when Error
    let default_when_error: char = char::from_str("a").unwrap_or('a');
}

fn error_propagation_verbose() -> Result<bool, ParseCharError> {
    // this function calls another, and instead to allow the called function deals with the error, we send the error to the calling function.

    // First, call and get Result
    let this_can_fail: Result<char, ParseCharError> = char::from_str("a");

    // Second, here takes action accordingly with the Result
    let output: char = match this_can_fail {
        Ok(character) => character,
        Err(e) => return Err(e), // premature return
    };

    Ok(output.is_ascii_control())
}

fn error_propagation_short() -> Result<bool, ParseCharError> {
    let this_can_fail: Result<char, ParseCharError> = char::from_str("a");
    // And alternative to the Match implementation. The operator ? calls the function from in From Trait.
    let character = this_can_fail?; // checks if Ok else return Err(e)
    Ok(character.is_ascii_control())
}
