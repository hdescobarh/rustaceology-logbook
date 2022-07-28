fn main() {
    //slices are reference to a part of a COLLECTION
    let a_string = String::from("Hola mundo!!! :)");
    println!("The original string is: '{}'", a_string);
    // A string slice stores on the stack a pointer to the index of the String where
    // the slice starts and the length of the slice
    let slice = string_slice_from(&a_string);
    println!("The slice taking index 0 to 4 is: '{}'", slice);
}

fn string_slice_from(a_string: &String) -> &str {
    &a_string[0..4]
}
