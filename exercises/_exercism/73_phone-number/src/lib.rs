const ALLOWED_NON_DIGITS: [char; 6] = [' ', '(', ')', '+', '-', '.'];
pub fn number(user_number: &str) -> Option<String> {
    let mut acc = String::with_capacity(10);
    for digit in user_number.chars() {
        // Check it is a allowed character
        if ALLOWED_NON_DIGITS.contains(&digit) {
            continue;
        } else if !digit.is_ascii_digit() {
            return None;
        };
        // Check digit is valid
        match digit {
            '0' if acc.is_empty() => return None,
            '1' if acc.is_empty() => (),
            '0'..='1' if acc.len() == 3 => return None,
            _ => acc.push(digit),
        }
    }
    if acc.len() == 10 { Some(acc) } else { None }
}
