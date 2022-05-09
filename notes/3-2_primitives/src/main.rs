fn type_of<T>(_: &T, message: &str) {
    println!("{}: {}", message, std::any::type_name::<T>())
}

fn main() {
    let default_float = 30.0;
    let default_int = 10;
    let unit_type = (); // the value is called "unit value"

    type_of(&default_float, "Default float type is"); // f64
    type_of(&default_int, "Default integer type is"); // i32
    type_of(&unit_type, "Unit value is"); // ()
}
