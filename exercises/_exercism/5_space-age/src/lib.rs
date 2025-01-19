#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s as f64 }
    }
}

pub trait Planet {
    fn get_orbital_period() -> Duration;
    fn years_during(d: &Duration) -> f64 {
        d.seconds / Self::get_orbital_period().seconds
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn get_orbital_period() -> Duration {
        Duration {
            seconds: 7600543.81992,
        }
    }
}
impl Planet for Venus {
    fn get_orbital_period() -> Duration {
        Duration {
            seconds: 19414149.052176,
        }
    }
}
impl Planet for Earth {
    fn get_orbital_period() -> Duration {
        Duration {
            seconds: 31557600.0,
        }
    }
}
impl Planet for Mars {
    fn get_orbital_period() -> Duration {
        Duration {
            seconds: 59354032.690079994,
        }
    }
}
impl Planet for Jupiter {
    fn get_orbital_period() -> Duration {
        Duration {
            seconds: 374355659.124,
        }
    }
}
impl Planet for Saturn {
    fn get_orbital_period() -> Duration {
        Duration {
            seconds: 929292362.8848,
        }
    }
}
impl Planet for Uranus {
    fn get_orbital_period() -> Duration {
        Duration {
            seconds: 2651370019.3296,
        }
    }
}
impl Planet for Neptune {
    fn get_orbital_period() -> Duration {
        Duration {
            seconds: 5200418560.032001,
        }
    }
}
