// Hashmaps are not in the prelude
use std::collections::HashMap;

fn main() {
    what_is_a_hashmap();
    let a_hashmap = create_new_hashmap();
    modifiying_a_hashmap(a_hashmap);
}

fn what_is_a_hashmap() {
    // f(key) -> index -> data structure
    println!("Hashmaps maps keys to indexes which points to some data structure (e.g. a linked list or a vector). The cost depends on both, the hash function and the used data structure; a good hash table have uniform collisions and around O(1)\n");

    // Rust implementation
    println!("The Rust default Hashing function is  SipHash 1-3, which protects against HashDoS attacks in exchange of a slightly worse performance with shorts and long keys.\n");
}

fn create_new_hashmap() -> HashMap<String, usize> {
    // must include the types if cannot be infered
    let an_empty_hashmap: HashMap<String, usize> = HashMap::new();
    an_empty_hashmap
}

fn modifiying_a_hashmap(mut a_hashmap: HashMap<String, usize>) {
    // adding a new key
    a_hashmap.insert(String::from("Answer"), 42);
    // Remember ownership rules
    let this_will_be_moved = String::from("Non-euclidean Pi"); // String implements Drop
    let this_will_be_copied = 3usize; // usize implements Copy
    a_hashmap.insert(this_will_be_moved, this_will_be_copied);
    // overwriting a hashmap
    a_hashmap.insert(String::from("Answer"), 5);
    // adding if not key
    let this_is_an_entry_enum = a_hashmap.entry(String::from("Answer"));
    let this_is_a_mutable_reference: &mut usize = this_is_an_entry_enum.or_insert(277);
    // you can perfom other operations such as modifiying the reference
    *this_is_a_mutable_reference += 100;
    assert!(105 == *a_hashmap.get("Answer").unwrap());
}
