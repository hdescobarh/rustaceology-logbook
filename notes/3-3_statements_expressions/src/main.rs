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
