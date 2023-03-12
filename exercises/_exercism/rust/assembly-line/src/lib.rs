pub fn production_rate_per_hour(speed: u8) -> f64 {
    let production_by_hour: f64 = 221.0;
    assert!(speed <= 10);
    let success_rate = {
        if speed <= 4 {
            1.0
        } else if speed <= 8 {
            0.9
        } else {
            0.77
        }
    };
    (speed as f64) * production_by_hour * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
