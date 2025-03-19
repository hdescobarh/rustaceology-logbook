use std::collections::HashMap;

#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

// Let w ∈ [0, max_weight], i ∈ [0, items.len()], maximum_value = value(w, i).
// Then, maximum_value = max{value(w - w_i, i-1) + v_i, value(w, i-1)}
pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut value: HashMap<(u32, usize), u32> = HashMap::new();
    for i in 1..=items.len() {
        for w in 1..=max_weight {
            value.insert((w, i), *value.get(&(w, i - 1)).unwrap_or(&0));
            if items[i - 1].weight <= w {
                let val =
                    value.get(&(w - items[i - 1].weight, i - 1)).unwrap_or(&0) + items[i - 1].value;
                if value[&(w, i)] < val {
                    value.insert((w, i), val);
                }
            }
        }
    }
    *value.get(&(max_weight, items.len())).unwrap_or(&0)
}
