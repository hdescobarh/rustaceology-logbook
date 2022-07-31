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
