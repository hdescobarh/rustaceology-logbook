pub fn is_valid_isbn(isbn: &str) -> bool {
    let (mut total, mut last) = (0, 0);
    for (index, letter) in isbn.chars().filter(|c| *c != '-').rev().enumerate() {
        last = index;
        match parse(index as u32, letter) {
            Some(value) => total += value,
            None => return false,
        }
    }
    total % 11 == 0 && last == 9
}

fn parse(index: u32, letter: char) -> Option<u32> {
    match (index, letter.to_digit(10)) {
        (0, None) if letter == 'X' => Some(10 * (index + 1)),
        (index, Some(d)) if index <= 9 => Some(d * (index + 1)),
        _ => None,
    }
}
