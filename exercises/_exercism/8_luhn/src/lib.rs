/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut valid_digits = 0_u32;
    let mut sum = 0_u32;
    for c in code.chars().rev() {
        if c.is_ascii_whitespace() {
            continue;
        }
        match c.to_digit(10).and_then(|v| v.checked_mul(2)) {
            Some(double) => {
                if valid_digits.rem_euclid(2) == 0 {
                    sum += double;
                } else {
                    sum += if double < 10 { double } else { double - 9 }
                }
                valid_digits += 1
            }
            None => return false,
        }
    }
    valid_digits > 1 && sum.checked_rem_euclid(10).is_some_and(|s| s == 0)
}
