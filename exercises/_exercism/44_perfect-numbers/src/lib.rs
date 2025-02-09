// Improved speed using prime factor decomposition O(sqrt(n))

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let aliquot_sum: u64 = sigma(num as u128)? - num;
    Some(match num.cmp(&aliquot_sum) {
        std::cmp::Ordering::Less => Classification::Abundant,
        std::cmp::Ordering::Equal => Classification::Perfect,
        std::cmp::Ordering::Greater => Classification::Deficient,
    })
}

// Calculates sigma [OEIS: A000203] and performs prime factor decomposition at the same time
fn sigma(num: u128) -> Option<u64> {
    if num == 0 {
        return None;
    }
    let (mut product, mut remaining) = (1, num);
    [2].into_iter()
        .chain((3..=num.isqrt() + 1).step_by(2))
        .for_each(|x| {
            let mut sum = 0;
            while remaining % x == 0 {
                sum = x * sum + x;
                remaining /= x;
            }
            product *= sum + 1;
        });

    if remaining > 2 {
        product *= remaining + 1;
    }
    u64::try_from(product).ok()
}
