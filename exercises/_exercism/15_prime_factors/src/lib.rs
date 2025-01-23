pub fn factors(mut n: u64) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();
    sieve_eratosthenes(n.isqrt() as usize).for_each(|prime| {
        while n % prime == 0 {
            output.push(prime);
            n /= prime;
        }
    });
    if n != 1 {
        output.push(n)
    }
    output
}

fn sieve_eratosthenes(upper_bound: usize) -> impl Iterator<Item = u64> {
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
        .skip(2)
        .filter_map(|(value, is_prime)| if is_prime { Some(value as u64) } else { None })
}
