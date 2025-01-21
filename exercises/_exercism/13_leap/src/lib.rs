pub fn is_leap_year(year: u64) -> bool {
    year.checked_rem(4).is_some_and(|v| v == 0)
        && match year.checked_rem(100) {
            Some(0) => year.checked_rem(400).is_some_and(|v| v == 0),
            Some(_) => true,
            None => false,
        }
}
