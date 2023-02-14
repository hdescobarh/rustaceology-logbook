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
#[allow(dead_code)]
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
    #[allow(dead_code)]
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

## 6-1_custom_data_type_enumeration

```rust
fn main() {
    // Variants implicit induces constructors
    let _ = EnumNamesHaveCamelcase::Variant1;
    // Variants can have associated values
    let my_enum_1 = EnumNamesHaveCamelcase::Variant2(String::from("This is a string"));
    println!("{:?}", my_enum_1);
    // Associated values can be any type; for example, Structs
    let my_enum_2 = EnumNamesHaveCamelcase::Variant3 {
        field1: 123,
        field2: 456,
    };
    // Associated functions with self argument (i.e "methods"; notice the "." syntax)
    println!("{}", my_enum_2.convert_to_string());
    // Other associated functions (notice the "::" syntax)
    let _ = EnumNamesHaveCamelcase::make_default(50usize, 20i128);
}

// Enums are useful for exhaustive and exclusive values; but are more powerfull because of the associated values
#[derive(Debug)]
enum EnumNamesHaveCamelcase {
    Variant1,                                 //doesn't have associated data
    Variant2(String),                         // is associated with Strings
    Variant3 { field1: usize, field2: i128 }, // is associated with a structure
}

// As Structures, Enumerations have associated functions (enum.associated_functions, and enum::associated_functions)
impl EnumNamesHaveCamelcase {
    fn convert_to_string(&self) -> String {
        // Pattern matching CONTROL FLOW
        match &&self {
            //This are the ARMS of the match. Each arm is compound of a PATTERN, the OPERATOR =>, and an EXPRESSION
            EnumNamesHaveCamelcase::Variant1 => String::from("No associated value"),
            EnumNamesHaveCamelcase::Variant2(variable_bind) => variable_bind.clone(), // notice here the patter binds the associated value to a new variable
            EnumNamesHaveCamelcase::Variant3 { field1, field2 } => {
                String::from("Field 1: ")
                    + &field1.to_string()
                    + &String::from("Field 2:")
                    + &field2.to_string()
            }
        }
    }

    fn make_default(x: usize, y: i128) -> EnumNamesHaveCamelcase {
        EnumNamesHaveCamelcase::Variant3 {
            field1: x,
            field2: y,
        }
    }
}
```

## 6-3_control_flow_match_and_if_let

```rust
fn main() {
    let a_number = Number::Odd(11);

    // Whith match
    match a_number {
        Number::Odd(value) => println!("With match: {}", value.rem_euclid(2) == 1),
        _ => (), // boiler plate, neccesary to ensure exhaustiviness check
    };

    // Whit if let
    if let Number::Odd(value) = a_number {
        println!("Withc if let: {}", value.rem_euclid(2) == 1)
    }
}

enum Number {
    Odd(i128),
    _Even(i128),
}
```

## 7_module_system

```rust
// module tree: package ⊇ crates ⊇ crate-tree := { root -> modules -> submodules -> paths}
fn main() {}
// modules/submodules compiler check order:
// (1) inline
// (2) file_module_name.rs
// (3) folder_module_name/mod.rs (old style)
pub mod big_module {

    // paths can be absolute (crate::item) or relative (self::item , super::item), the decision depends on the project, but as rule of thumb, use absolute
    fn path_types() {
        // absolute path
        crate::big_module::do_something_one();
        // relative path
        super::big_module::do_something_one();
        // another relative path, this function is at the same level inside the module tree than submod_children_privacity
        submod_children_privacity::public_function();
    }

    // by default, items inside CHILDREN are private to parent, must use 'pub' keyword to make them accesible
    pub mod submod_children_privacity {
        // by default, fields inside a public struct are private
        pub struct Astruct {
            field1: bool,
            field2: bool,
        }
        // by default, variants of a public enum are public
        pub enum AnEnum {
            variantA,
            variantB,
        }

        pub fn public_function() {}
    }

    // items in child modules can use the items in their ancestor modules.
    fn do_something_one() {}

    mod submod_parent_privacity {
        fn use_function_in_parent() {
            // use make shortcuts to avaiable paths in the scope
            use super::do_something_one;
            do_something_one()
        }
    }

    mod use_and_idiomatic_paths {
        // to bring function paths: 'use path::item'
        use crate::big_module;
        fn use_shortcut() {
            // when calling it: 'item::function'
            big_module::do_something_one();
        }
        // to bring other items paths (e.g. structs, enums): 'use path::other'
        use crate::big_module::submod_children_privacity::Astruct;

        // An exception to the later is whith repeated names; in such cases use the same idiomatic than bringing functions or rename with the 'as' keyword
        use std::fmt::Result;
        use std::io::Result as IoResult;

        // the later 'use' examples bring PRIVATE names, to make them public you can add 'pub' keyword
        fn re_exporting_technique() {
            pub use crate::big_module as alias;
            fn use_re_exported() {
                alias::do_something_one();
            }
        }
    }
}

// re-exporting names: pub use path:item

pub fn glob_operator() {
    //glob operator brings every that is public
    use crate::big_module::submod_children_privacity::*;
    // AnEnum is in submod_children_privacity wich is in big_module
    let x = AnEnum::variantA;
}
```

## 8-1_collections_vectors

```rust
fn main() {
    // In a vector, values are next to each other in MEMORY

    // The are structs, have values on the heap, so when they are out of scope, calls drop

    fn create_a_new_vector() {
        // Vector implements generics: Vec<T>, so the type must be specified for a empty vector
        let mut this_is_a_vector: Vec<usize> = Vec::new();
        // It can be initialized with data
        this_is_a_vector = vec![0usize, 3usize];
    }

    fn adding_data_to_a_vector(mut this_is_a_vector: Vec<usize>) {
        // updating a vector
        this_is_a_vector.push(42usize);
        // adding by index
        this_is_a_vector[1] = 3usize;
    }

    fn reading_data_from_a_vector(this_is_a_vector: Vec<usize>) {
        // reading by index can panic if out of lenght
        let mut value = this_is_a_vector[1];
        // reading by method returns Option<T>
        value = match this_is_a_vector.get(1) {
            Some(val) => *val,
            None => 0,
        }
    }

    fn iterating_over_vectors() {
        let mut fibo: Vec<usize> = vec![0, 1, 1, 2, 3, 5, 8, 13];
        // you can iterate over unmutable or mutable references
        for value in &mut fibo {
            *value += 1;
        }
    }

    fn storing_multiple_types() {
        // use Enum and in each variant represent each type
        enum thisIsAnEnum {
            variantA(u8),
            variantB(u32),
            variantC(String),
        }
        let this_is_a_vector: Vec<thisIsAnEnum> = Vec::new();
    }
}
```

## 8-2_collections_strings

```rust
use std::str;
use std::vec;

fn main() {
    println!("The only string type in the CORE language is &str (string slices), which are references to UTF-8 encoded data\n");
    // String collection vs String references
    string_vs_string_slices();
    // Ambstract character, stream of bytes and graphemes
    string_for_computer_vs_human_intuition();
    // Accesing to data
    indexing_and_slicing();
    // iterating with characters or bytes
    iterating_over_strings();
}

fn string_vs_string_slices() {
    let this_is_a_string_slice_from_literal: &str =
        "This is a String literal, it is stored in the program binary and we get a reference to that data";

    let this_is_a_String: String = String::from(
        "String type is a wrapper around a vector of UTF-8 encoded codepoints; in other words Strings are Vec<u8> of valid Unicode Scalar Values",
    );

    let greek_small_epsilon = String::from("ε"); // this is the encoded character U+03B5
    let greek_small_epsilon_codepoint = 0x03B5; // 949

    println!("Remember that an encoded character maps to a integer value wich is represented as a stream of bytes following some rules of an encoding schema; for example, {greek_small_epsilon} has the codepoint {greek_small_epsilon_codepoint} and in UTF-8 encoding is the chain: {:?}\n", greek_small_epsilon.as_bytes());
}

fn string_for_computer_vs_human_intuition() {
    let small_letter_i_with_acute = "í".to_string(); // codepoint U+00ED
    let small_letter_i_plus_combining_acute_accent_bytestream = vec![105, 204, 129]; // codepoint U+0069 + codepoint U+0301

    let combined_small_letter_i_with_acute =
        str::from_utf8(&small_letter_i_plus_combining_acute_accent_bytestream).unwrap();

    let len1 = small_letter_i_with_acute.chars().count(); // 1
    let len2 = combined_small_letter_i_with_acute.chars().count(); // 2

    let stream1 = small_letter_i_with_acute.as_bytes(); // [195, 173]
    let stream2 = combined_small_letter_i_with_acute.as_bytes(); // [105, 204, 129]

    println!("The glyph of {small_letter_i_with_acute} is the same that {combined_small_letter_i_with_acute}. For a human they are the same GRAPHEME, but for a computer they are two separated objects in at least two senses: represent different UNICODE SCALAR VALUES and have different STREAM OF BYTES.\n");

    println!("The first one (U+00ED), has the bytestream {stream1:?} and has only {len1} scalar unicode value; in contrast, the the bytestream of the later (U+0069 + U+0301) is {stream2:?} and has {len2} scalar unicode values.\n");
}

fn indexing_and_slicing() {
    println!("Remember that in UTF-8 a codepoint can have 1 to 4 bytes, so indexing potentialy can give a invalid value that does not correspond to any Scalar Unicode Value. In rust you cannot index, but can get slice, so use it with caution.\n")
}

fn iterating_over_strings() {
    println!("You can iterate over a String as a chain of char type (i.e. as Scalar Unicode Values) or as a chain of bytes. There is not methods in the standard library for graphemes.\n")
}
```
