use std::collections::HashMap;

#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut value: HashMap<(u32, usize), u32> = HashMap::new();
    for i in 1..=items.len() {
        for bag_size in 1..=max_weight {
            let option1 = *value.get(&(bag_size, i - 1)).unwrap_or(&0);
            let item = &items[i - 1];
            let option2 = bag_size
                .checked_sub(item.weight)
                .map(|prior| value.get(&(prior, i - 1)).unwrap_or(&0) + item.value)
                .unwrap_or_default();
            value.insert((bag_size, i), option1.max(option2));
        }
    }
    *value.get(&(max_weight, items.len())).unwrap_or(&0)
}
