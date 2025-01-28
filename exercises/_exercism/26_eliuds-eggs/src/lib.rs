pub fn egg_count(mut display_value: u32) -> usize {
    let mut counter = 0;
    while display_value > 1 {
        counter += display_value % 2;
        display_value /= 2
    }
    (counter + display_value) as usize
}
