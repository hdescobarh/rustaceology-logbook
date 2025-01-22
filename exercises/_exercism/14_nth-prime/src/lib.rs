pub fn nth(n: u32) -> u32 {
    let upper_bound = get_upper_bound(n);
    sieve_eratosthenes(upper_bound).nth(n as usize + 2).unwrap() as u32
}

// p(n) < n ln(n + 2) n log log n for n > 3 (doi: 10.1112/plms/s2-45.1.21)
fn get_upper_bound(n: u32) -> usize {
    let k = (n + 1) as f64;
    if n <= 3 {
        6
    } else {
        (k * k.ln() + 2.0 * k * k.ln().ln()) as usize
    }
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
