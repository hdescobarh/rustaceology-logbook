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
            value.insert((bag_size, i), *value.get(&(bag_size, i - 1)).unwrap_or(&0));
            let item = &items[i - 1];
            if item.weight <= bag_size {
                let maybe_higher =
                    value.get(&(bag_size - item.weight, i - 1)).unwrap_or(&0) + item.value;
                if value[&(bag_size, i)] < maybe_higher {
                    value.insert((bag_size, i), maybe_higher);
                }
            }
        }
    }
    *value.get(&(max_weight, items.len())).unwrap_or(&0)
}
