/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut valid_digits = 0_u32;
    let mut sum = 0_u32;

    for c in code.chars().rev() {
        if c.is_ascii_whitespace() {
            continue;
        }
        match c.to_digit(10) {
            Some(value) => {
                if valid_digits.rem_euclid(2) == 0 {
                    sum += value;
                } else {
                    sum += if value < 5 { value } else { value - 9 }
                }
                valid_digits += 1
            }
            None => return false,
        }
    }

    valid_digits > 1 && sum.checked_rem_euclid(10).is_some_and(|v| v == 0)
}
