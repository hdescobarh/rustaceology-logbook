pub fn nth(n: u32) -> u32 {
    // π(x) < 1.25506 x/log x for 1 < x (doi:10.1215/ijm/1255631807)
    if n > 243023858 {
        panic!("The {}nth prime is bigger than u32::MAX", n)
    }
    // For n <= 243023858 the cast to u32 will never overflows
    let upper_bound = (1.25506 * n as f64 / (n as f64).ln()).ceil() as u32;

    sieve_eratosthenes(upper_bound).unwrap()
}

fn sieve_eratosthenes(n: u32) -> Option<u32> {
    todo!()
}
