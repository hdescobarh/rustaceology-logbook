use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(2_u64..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exp(b_pub, a, p)
}

fn modular_exp(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    for _ in 0..exponent {
        result = (base * result).rem_euclid(modulus)
    }
    result
}
