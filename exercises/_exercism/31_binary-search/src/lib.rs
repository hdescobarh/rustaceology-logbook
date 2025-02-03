// Returns the first occurrence of key in array
fn binary_search_recursive<A: AsRef<[T]>, T: PartialOrd + Sized>(
    array: &A,
    key: &T,
    start: usize,
    end: usize,
    current_position: Option<usize>,
) -> Option<usize> {
    if start > end {
        return current_position;
    }
    let mid_point = end.checked_add(start)?.checked_div(2)?;

    if array.as_ref()[mid_point] < *key {
        binary_search_recursive(array, key, mid_point.checked_add(1)?, end, current_position)
    } else if array.as_ref()[mid_point] > *key {
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

pub fn find<A: AsRef<[T]>, T: PartialOrd>(array: A, key: T) -> Option<usize> {
    binary_search_recursive(&array, &key, 0, array.as_ref().len().checked_sub(1)?, None)
}
