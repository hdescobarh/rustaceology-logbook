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
