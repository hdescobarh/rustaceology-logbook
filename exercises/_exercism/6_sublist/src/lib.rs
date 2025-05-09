#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if is_superlist(first_list, second_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    first_list.is_empty() && !second_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|window| window == first_list)
}

fn is_superlist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    !first_list.is_empty() && second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|window| window == second_list)
}
