pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let exponent: u32 = num_str.len().try_into().unwrap();
    num == num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .fold(0, |acc, value: u32| value.pow(exponent) + acc)
}
