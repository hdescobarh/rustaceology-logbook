pub fn nth(n: u32) -> u32 {
    let upper_bound = 150_000;
    sieve_eratosthenes(upper_bound).nth(n as usize + 2).unwrap() as u32
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
