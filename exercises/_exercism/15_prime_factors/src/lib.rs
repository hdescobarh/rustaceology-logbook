pub fn factors(n: u64) -> Vec<u64> {
    todo!("This should calculate the prime factors of {n}")
}

fn sieve_eratosthenes(upper_bound: usize) -> impl Iterator<Item = usize> {
    let mut check_list = vec![true; upper_bound + 3];
    for i in 2..=upper_bound.isqrt() {
        if check_list[i] {
            for j in (i.pow(2)..=upper_bound).step_by(i) {
                check_list[j] = false;
            }
        }
    }
    check_list
        .into_iter()
        .enumerate()
        .filter_map(|(value, is_prime)| if is_prime { Some(value) } else { None })
}
