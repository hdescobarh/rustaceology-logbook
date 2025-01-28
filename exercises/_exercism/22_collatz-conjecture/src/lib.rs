pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut counter = 0_u64;
    while n != 1 {
        n = match n.checked_rem_euclid(2) {
            Some(0) => n / 2,
            Some(_) => n.checked_mul(3).and_then(|v| v.checked_add(1))?,
            None => return None,
        };
        counter = counter.checked_add(1)?
    }
    Some(counter)
}
