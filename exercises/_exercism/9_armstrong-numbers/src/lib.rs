pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let exponent: u32 = digits.len().try_into().unwrap();
    digits.into_iter().fold(0, |acc, v| v.pow(exponent) + acc) == num
}
