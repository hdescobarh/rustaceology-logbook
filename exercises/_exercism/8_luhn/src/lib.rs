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
                    sum += double_and_lower_than_nine(value)
                }
                valid_digits += 1
            }
            None => return false,
        }
    }

    valid_digits > 1 && sum.rem_euclid(10) == 0
}

fn double_and_lower_than_nine(value: u32) -> u32 {
    if value < 5 {
        value * 2
    } else {
        value * 2 - 9
    }
}
