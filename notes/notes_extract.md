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
