fn main() {
    //slices are reference to a part of a COLLECTION
    let a_string = String::from("Hola mundo!!! :)");
    // A string slice stores on the stack a pointer to the index of the String where
    // the slice starts and the length of the slice
    let slice = string_slice_from(&a_string);
    println!("{}", slice)
}

fn string_slice_from(a_string: &String) -> &str {
    &a_string[0..4]
}
