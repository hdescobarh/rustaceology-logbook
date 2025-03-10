const ALLOWED_NON_DIGITS: [char; 6] = [' ', '(', ')', '+', '-', '.'];
pub fn number(user_number: &str) -> Option<String> {
    user_number
        .chars()
        .filter_map(|c| {
            if c.is_ascii_digit() {
                Some(Ok(c))
            } else if ALLOWED_NON_DIGITS.contains(&c) {
                None
            } else {
                Some(Err(()))
            }
        })
        .try_fold(String::with_capacity(10), |mut acc, maybe_digit| {
            maybe_digit.and_then(|digit| {
                match digit {
                    '1' if acc.is_empty() => (),
                    '0' if acc.is_empty() => return Err(()),
                    '0'..='1' if acc.len() == 3 => return Err(()),
                    _ => acc.push(digit),
                };
                Ok(acc)
            })
        })
        .map(|number| {
            if number.len() == 10 {
                Some(number)
            } else {
                None
            }
        })
        .unwrap_or_default()
}
