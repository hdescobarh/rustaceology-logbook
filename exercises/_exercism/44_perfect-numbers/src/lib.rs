#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let aliquot_sum: u64 = (1..num).filter(|n| num % n == 0).sum();
    Some(match num.cmp(&aliquot_sum) {
        std::cmp::Ordering::Less => Classification::Abundant,
        std::cmp::Ordering::Equal => Classification::Perfect,
        std::cmp::Ordering::Greater => Classification::Deficient,
    })
}
