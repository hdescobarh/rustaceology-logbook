pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let upper_bound = match usize::try_from(upper_bound) {
        Ok(v) if v > 1 => v,
        _ => return Vec::new(),
    };
    let mut index_is_prime = vec![true; upper_bound + 1];
    (0..2).for_each(|i| index_is_prime[i] = false);
    (2..=upper_bound.isqrt()).for_each(|number| {
        if index_is_prime[number] {
            for multiple in (number.pow(2)..=upper_bound).step_by(number) {
                index_is_prime[multiple] = false;
            }
        }
    });
    index_is_prime
        .into_iter()
        .enumerate()
        .filter(|(_, is_prime)| *is_prime)
        .map(|(number, _)| number as u64)
        .collect()
}
