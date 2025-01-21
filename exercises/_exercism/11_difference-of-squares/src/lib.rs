use std::ops::Div;

pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1)).div(2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (n * (n + 1) * (2 * n + 1)) / 6
}

pub fn difference(n: u32) -> u32 {
    (n * (n + 1) * (3 * n + 2) * (n - 1)) / 12
}
