fn main() {
    let mut original_string: String = String::from("Hola");

    // creating a reference is called 'borrowing'
    borrowing_default(&original_string);
    println!("'original string' ({}) is still accesible", original_string);

    // In a given scope can only exist ONE mutable reference to a piece of data,
    //also, its use with  immutable references have restrictions to avoid "data race"
    borrowing_mutable(&mut original_string);
    println!(
        "'other_borrow' is a mutable reference to 'original_string',\
     and from that reference we changed the original value to: '{}'",
        original_string
    );

    dangling_references();
}

fn borrowing_default(reference: &String) {
    println!(
        "'reference' points to the pointer of 'original_string' which points to '{}'",
        reference
    )
} // since 'reference' does not own the String, when it goes out of scope the value is not dropped

fn borrowing_mutable(reference: &mut String) {
    reference.push_str(", mundo");
}

fn dangling_references() {
    println!("Rust compiler guarantees that will never be dangling pointers")
}
