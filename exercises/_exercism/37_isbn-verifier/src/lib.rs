pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut total = 0;
    let mut last_index = 0;
    for (index, c) in isbn.chars().filter(|c| *c != '-').rev().enumerate() {
        last_index = index;
        match parse(c, index as u32) {
            Some(value) => total += value,
            None => return false,
        }
    }
    total % 11 == 0 && last_index == 9
}

fn parse(c: char, index: u32) -> Option<u32> {
    match (index, c.to_digit(10)) {
        (0, None) => {
            if c == 'X' {
                Some(10 * (index + 1))
            } else {
                None
            }
        }
        (index, Some(d)) if index <= 9 => Some(d * (index + 1)),
        _ => None,
    }
}
