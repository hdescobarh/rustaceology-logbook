use std::convert::From;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: u8,   // hours are integers ∈ [0, 23] (modulo 24)
    minutes: u8, // minutes are integers ∈ [0, 59] (modulo 60)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (normal_minutes, hours_to_add) = Self::normalize_minutes(minutes);
        let normal_hours = Self::normalize_hours(hours + hours_to_add);

        Self {
            hours: normal_hours,
            minutes: normal_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (normal_minutes, hours_to_add) = Self::normalize_minutes(self.minutes as i32 + minutes);
        let normal_hours = Self::normalize_hours(self.hours as i32 + hours_to_add);
        Self {
            minutes: normal_minutes,
            hours: normal_hours,
        }
    }

    fn normalize_hours(hours: i32) -> u8 {
        hours.rem_euclid(24) as u8
    }

    fn normalize_minutes(minutes: i32) -> (u8, i32) {
        (minutes.rem_euclid(60) as u8, minutes.div_euclid(60))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl From<Clock> for String {
    fn from(value: Clock) -> Self {
        format!("{:0>2}:{:0>2}", value.hours, value.minutes)
    }
}
