# My notes extract


## 3-1_variables

```rust
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
```

## 3-2_primitives

```rust
fn type_of<T>(_: &T, message: &str) {
    println!("{}: {}", message, std::any::type_name::<T>())
}

fn escalar_types() {
    let default_float = 30.0;
    let default_int = 10;
    let unit_type = ();
    // "A char is a ‘Unicode scalar value’, which is any ‘Unicode code point’ other than a surrogate code point." https://doc.rust-lang.org/std/primitive.char.html
    let default_character = 'Ф'; // Ф code point is U+0424, is encoded as 0xd0a4 in UTF-8

    println!("Some examples of scalar types:");
    type_of(&default_float, "Default float type is"); // f64
    type_of(&default_int, "Default integer type is"); // i32
    type_of(&unit_type, "Unit value is"); // ()
    type_of(&default_character, "This type have a size of 4 bytes"); // char
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
```

## 3-3_statements_expressions

```rust
// statements give instructions

fn function_definition_is_a_statement() -> i32 {
    // if implicit return (i.e. ends with an expression), you must declare return TYPE

    let variable_assignment_is_a_statement = 42;

    variable_assignment_is_a_statement //ending without semicolon makes it an expression
}

fn main() {
    // expresions are evaluate

    let function_calling_are_expressions = function_definition_is_a_statement();

    let block_scopes_are_expressions = {
        let x = 5;
        x + 1
    };

    let this_block_evaluates_to_unit_value = {
        let _y = 5; //unused variable
    };

    println!("macro calling is an expression");

    println!("{}", function_calling_are_expressions);
    println!("{}", block_scopes_are_expressions);
    println!("{:?}", this_block_evaluates_to_unit_value);
}
```

## 3-5_controlflow_if

```rust
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
```

## 3-5_controlflow_loops

```rust
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
```

## 4-1_ownership

```rust
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
```

## 4-2_references_and_borrowing

```rust
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
```

## 4-3_slice_reference

```rust
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
```

## 5_custom_data_type_Structure

```rust
fn main() {
    // Introducing structs
    basics_struct();

    // Tuple structs: as structs but without names for the fields
    struct Coordinates(i64, i64, u64);
    let _vector = Coordinates(14, 15, 9);

    // Unit-Like structs: as tuple structs, but whithout fields
    struct NullElement;
    let _null_element = NullElement;
}

fn basics_struct() {
    // Structure instantiation syntax
    let population1 = Population {
        genre: String::from("Homo"),
        specie: String::from("sapiens"),
        size: 7000,
    };

    // Structure instantiation shorthand
    let population2 =
        structure_instantiation_shorthand(String::from("Homo"), String::from("habilis"), 0u32);

    // update with move
    let _population3 = Population {
        specie: String::from("neanderthalensis"),
        ..population2 // COPY or MOVE the remaining fields. String is on the heap, so it is moved
    };

    println!("population2.genre was moved; then, population2 is no longer accesible");

    // update with copy
    let population4 = Population {
        genre: String::from("Pan"),
        specie: String::from("fictitious"),
        ..population1 // COPY or MOVE the remaining fields. u32 is only-stack data, so it is copied
    };

    println!(
        "Population 1:\n{:?}\nPopulation 4:\n{:?}\n",
        population1, population4
    );

    println!("population1 is extinct: {}", population1.it_is_extinct())
}

// Structure definition
#[derive(Debug)]
struct Population {
    genre: String,
    specie: String,
    size: u32,
}

// Associated functions
impl Population {
    // METHODS are associated functions which requires an instance of the struct (i.e. uses &self)
    fn it_is_extinct(&self) -> bool {
        // '&self' is a shorthand for 'self: &self'
        if self.size > 0 {
            false
        } else {
            true
        }
    }
    // associated functions which are not methods usually are used as constructors
    fn homosapiens(size: u32) -> Population {
        Population {
            genre: String::from("Homo"), //String::from() itself is an example of constructor. Notice the "::"
            specie: String::from("sapiens"),
            size,
        }
    }
}

fn structure_instantiation_shorthand(genre: String, specie: String, size: u32) -> Population {
    Population {
        genre: genre, // Use complete sintax
        specie,       // or use field init short hand
        size,
    }
}
```
