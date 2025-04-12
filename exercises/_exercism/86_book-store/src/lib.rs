use std::collections::HashMap;

const UNITARY_PRICE: f64 = 800.0;

pub fn lowest_price(books: &[u32]) -> u32 {
    todo!("Find the lowest price of the bookbasket with books {books:?}")
}

fn count_by_id(books: &[u32]) -> Vec<usize> {
    let mut frequency_by_id: HashMap<&u32, usize> = HashMap::new();
    for id in books {
        *frequency_by_id.entry(id).or_default() += 1;
    }
    frequency_by_id.values().copied().collect()
}
