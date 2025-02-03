pub fn find<T: PartialOrd>(array: &[T], key: T) -> Option<usize> {
    binary_search_recursive::<T>(array, &key, 0, array.len(), None)
}

// Returns the first occurrence of key in array
fn binary_search_recursive<T: PartialOrd>(
    array: &[T],
    key: &T,
    start: usize,
    end: usize,
    current_position: Option<usize>,
) -> Option<usize> {
    if start > end {
        return current_position;
    }
    let mid_point = end.checked_add(start)?.checked_div(2)?;

    if array.get(mid_point)? < key {
        binary_search_recursive(array, key, mid_point.checked_add(1)?, end, current_position)
    } else if array.get(mid_point)? > key {
        match mid_point.checked_sub(1) {
            Some(end) => binary_search_recursive(array, key, start, end, current_position),
            None => current_position,
        }
    } else {
        match mid_point.checked_sub(1) {
            Some(end) => binary_search_recursive(array, key, start, end, Some(mid_point)),
            None => Some(mid_point),
        }
    }
}
