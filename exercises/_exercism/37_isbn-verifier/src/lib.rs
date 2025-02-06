pub fn is_valid_isbn(isbn: &str) -> bool {
    let (mut total, mut is_complete) = (0, false);
    for (index, letter) in isbn.chars().filter(|c| *c != '-').rev().enumerate() {
        is_complete |= index == 9;
        match (index, letter.to_digit(10)) {
            (0, None) if letter == 'X' => total += 10 * (index as u32 + 1),
            (0..=9, Some(d)) => total += d * (index as u32 + 1),
            _ => return false,
        }
    }
    total % 11 == 0 && is_complete
}
