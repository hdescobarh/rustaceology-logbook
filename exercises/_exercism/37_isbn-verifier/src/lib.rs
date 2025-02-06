pub fn is_valid_isbn(isbn: &str) -> bool {
    let (mut total, mut is_complete) = (0, false);
    for (index, letter) in isbn.chars().filter(|c| *c != '-').rev().enumerate() {
        is_complete |= index == 9;
        match parse(index as u32, letter) {
            Some(value) => total += value,
            None => return false,
        }
    }
    total % 11 == 0 && is_complete
}

fn parse(index: u32, letter: char) -> Option<u32> {
    match (index, letter.to_digit(10)) {
        (0, None) if letter == 'X' => Some(10 * (index + 1)),
        (0..=9, Some(d)) => Some(d * (index + 1)),
        _ => None,
    }
}
