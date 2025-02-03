use binary_search::*;

#[test]
fn finds_a_value_in_an_array_with_one_element() {
    assert_eq!(find(&[6], 6), Some(0));
}

#[test]
fn finds_a_value_in_the_middle_of_an_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 6), Some(3));
}

#[test]
fn finds_a_value_at_the_beginning_of_an_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 1), Some(0));
}

#[test]
fn finds_a_value_at_the_end_of_an_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 11), Some(6));
}

#[test]
fn finds_a_value_in_an_array_of_odd_length() {
    assert_eq!(
        find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144),
        Some(9)
    );
}

#[test]
fn finds_a_value_in_an_array_of_even_length() {
    assert_eq!(
        find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], 21),
        Some(5)
    );
}

#[test]
fn identifies_that_a_value_is_not_included_in_the_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 7), None);
}

#[test]
fn a_value_smaller_than_the_array_s_smallest_value_is_not_found() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 0), None);
}

#[test]
fn a_value_larger_than_the_array_s_largest_value_is_not_found() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 13), None);
}

#[test]
fn nothing_is_found_in_an_empty_array() {
    assert_eq!(find(&[], 1), None);
}

#[test]
fn nothing_is_found_when_the_left_and_right_bounds_cross() {
    assert_eq!(find(&[1, 2], 0), None);
}

#[test]
#[cfg(feature = "generic")]
fn works_for_arrays() {
    assert_eq!(find([6], 6), Some(0));
}

#[test]
#[cfg(feature = "generic")]
fn works_for_vec() {
    let vector = vec![6];
    assert_eq!(find(&vector, 6), Some(0));
    assert_eq!(find(vector, 6), Some(0));
}

#[test]
#[cfg(feature = "generic")]
fn works_for_str_elements() {
    assert_eq!(find(["a"], "a"), Some(0));
    assert_eq!(find(["a", "b"], "b"), Some(1));
}

#[test]
fn manages_correctly_array_with_duplicates() {
    let array = [2, 4, 4, 4, 7, 7, 9];
    let cases = [
        (9, Some(6)),
        (4, Some(1)),
        (5, None),
        (2, Some(0)),
        (1, None),
        (42, None),
    ];
    for (input, expected) in cases {
        assert_eq!(find(array, input), expected);
    }
}
