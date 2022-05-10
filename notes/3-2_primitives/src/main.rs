fn type_of<T>(_: &T, message: &str) {
    println!("{}: {}", message, std::any::type_name::<T>())
}

fn escalar_types() {
    let default_float = 30.0;
    let default_int = 10;
    let unit_type = (); // the value is called "unit value"

    println!("Some examples of scalar types:");
    type_of(&default_float, "Default float type is"); // f64
    type_of(&default_int, "Default integer type is"); // i32
    type_of(&unit_type, "Unit value is"); // ()
}

fn compound_types() {
    let this_is_a_tuple: (i32, f64, char) = (3, 3.14159, '🐉');
    let this_is_an_array: [i32; 4] = [1, 5, 20, 60]; // array only one type and lenght fixed at compilation time

    let (element1, element2, element3) = this_is_a_tuple;

    println!(
        "I DESTRUCTURED the tuple to print: ({}, {},{})",
        element1, element2, element3
    );
    println!("The 1rd element of the tuple: {}", this_is_a_tuple.2);
    println!("The 3rd element of the array {}", this_is_an_array[2]);
}

fn main() {
    escalar_types();
    compound_types();
}
