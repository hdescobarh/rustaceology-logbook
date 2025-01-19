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
    // Check necessary condition
    if first_list.len() >= second_list.len() {
        return false;
    }

    // Empty list case
    if first_list.is_empty() && !second_list.is_empty() {
        return true;
    }

    // Non-empty list case
    let free_spaces = second_list.len() - first_list.len();
    let mut index = 0;

    while index <= free_spaces {
        if let Some(position) = second_list[index..]
            .iter()
            .position(|value| *value == first_list[index])
        {
            if second_list[position..position + first_list.len()] == *first_list {
                return true;
            } else {
                index = position + 1;
                continue;
            }
        }
        return false;
    }

    false
}

fn is_superlist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    // Check necessary condition
    if first_list.len() <= second_list.len() {
        return false;
    }

    // Empty list case
    if !first_list.is_empty() && second_list.is_empty() {
        return true;
    }

    // Non-empty list case
    let free_spaces = first_list.len() - second_list.len();
    let mut index = 0;

    while index <= free_spaces {
        if let Some(position) = first_list[index..]
            .iter()
            .position(|value| *value == second_list[index])
        {
            if first_list[position..position + second_list.len()] == *second_list {
                return true;
            } else {
                index = position + 1;
                continue;
            }
        }
        return false;
    }

    false
}
