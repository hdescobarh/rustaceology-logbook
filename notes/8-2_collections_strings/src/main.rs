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
