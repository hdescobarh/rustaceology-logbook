fn main() {
    // Two data structures:
    // 1. stack (last in, first out) -> known size at compile time | pushing onto the stack - popping off the stack
    // 2. heap (almost complete tree + heap property) -> unknown size at compile time | allocating on the heap _returns_ pointers

    // implications of ownership rules:
    strings_and_memory();
    variable_move();
    variable_clone();
    variable_copy();
    let string_to_be_moved = String::from("I will be moved");
    let _now_i_own_the_return = ownership_transfer_in_functions(string_to_be_moved);
}

fn strings_and_memory() {
    let _literal_string = "hello"; // known size at compile time + fixed size -> on the stack

    {
        let mut string_type = String::from("hello"); // unknown size at compile time + mutable (variable size) -> on the heap
        println!("{}", string_type);
        string_type.push_str(", world");
        print!("{}", string_type)
    } // this scope is over, the variable "string_type" is no longer valid https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop
}

fn variable_move() {
    // differs from others languages "shallow copy" because the first variable is invalidated

    let initial_string = String::from("hello");
    println!("initial_string is binded to '{}', strings have values on the stack (e.g. pointers) and on the heap (i.e. the actual content)", initial_string);
    let other_string = initial_string; // initial_string is moved to other_string
    println!(
        "other_string copied values on the stack and now owns the values '{}' on the heap. initial_strin is no longer accesible",
        other_string
    )
}

fn variable_clone() {
    // this metod allows to do a "deep copy" (i.e. to also make a copy of the heap)

    let initial_string = String::from("hello");
    println!("initial_string is binded to '{}'", initial_string);
    let other_string = initial_string.clone(); // initial_string is cloned to other_string
    println!(
        "Both, initial string '{}' and other_string '{}' are accesible",
        initial_string, other_string
    )
}

fn variable_copy() {
    // important: drop and copy traits are exclusive https://doc.rust-lang.org/std/ops/trait.Drop.html#copy-and-drop-are-exclusive

    // data enterely on the stack doesn't need to be moved.

    let stack_only_data = 5;
    let copy_of_stack_only_data = stack_only_data;
    println!(
        "Both are accesible. stack_only_data = {}, copy_of_stack_only_data = {}",
        stack_only_data, copy_of_stack_only_data
    );
}

fn ownership_transfer_in_functions(take_ownership_of: String) -> String {
    // using variables as arguments passes the ownership to the function; i.e. the variable is copied (if only stack data) or moved
    take_ownership_of // returning a value transfers ownership
}
