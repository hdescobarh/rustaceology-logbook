pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return Period::null().into();
    }
    let (mut current, mut periods) = (0, Vec::new());
    while n > 0 {
        if let Some(p) = Period::new(n % 1000, current) {
            periods.push(p);
        }
        n /= 1000;
        current += 1;
    }
    periods
        .into_iter()
        .rev()
        .map(String::from)
        .collect::<Vec<String>>()
        .join(" ")
}

struct Period {
    name: u8,
    ones: u8,
    tens: u8,
    hundreds: u8,
}

impl Period {
    pub fn new(mut value: u64, name: u8) -> Option<Self> {
        if value > 999 || name > 6 {
            return None;
        }
        let mut period_places: Vec<u8> = Vec::with_capacity(3);
        for _ in 0..3 {
            period_places.push((value % 10) as u8);
            value /= 10;
        }
        if period_places.iter().sum::<u8>() == 0 {
            return None;
        }
        Some(Period {
            name,
            ones: period_places[0],
            tens: period_places[1],
            hundreds: period_places[2],
        })
    }

    pub fn null() -> Self {
        Self {
            name: 0,
            ones: 0,
            tens: 0,
            hundreds: 0,
        }
    }

    fn name_to_word(&self) -> &str {
        match self.name {
            1 => " thousand",
            2 => " million",
            3 => " billion",
            4 => " trillion",
            5 => " quadrillion",
            6 => " quintillion",
            _ => "",
        }
    }

    fn places_to_word(&self) -> String {
        match (self.hundreds, self.tens, self.ones) {
            (0, 0, ones) => Self::digit_to_word(ones).to_string(),
            (0, _, _) => self.tens_to_word().to_string(),
            (_, 0, 0) => self.hundreds_to_word(),
            (_, 0, ones) => format!("{} {}", self.hundreds_to_word(), Self::digit_to_word(ones)),
            (_, _, _) => format!("{} {}", self.hundreds_to_word(), self.tens_to_word()),
        }
    }

    fn digit_to_word(digit: u8) -> &'static str {
        match digit {
            0 => "zero",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            _ => "",
        }
    }

    fn tens_to_word(&self) -> String {
        match (self.tens, self.ones) {
            (1, 0) => "ten".to_string(),
            (1, 1) => "eleven".to_string(),
            (1, 2) => "twelve".to_string(),
            (1, 3) => "thirteen".to_string(),
            (1, 4) => "fourteen".to_string(),
            (1, 5) => "fifteen".to_string(),
            (1, 6) => "sixteen".to_string(),
            (1, 7) => "seventeen".to_string(),
            (1, 8) => "eighteen".to_string(),
            (1, 9) => "nineteen".to_string(),
            (_, 0) => self.tens_prefix_from_twenty().to_string(),
            _ => format!(
                "{}-{}",
                self.tens_prefix_from_twenty(),
                Self::digit_to_word(self.ones)
            ),
        }
    }

    fn tens_prefix_from_twenty(&self) -> &str {
        match self.tens {
            2 => "twenty",
            3 => "thirty",
            4 => "forty",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            _ => "",
        }
    }

    fn hundreds_to_word(&self) -> String {
        format!("{} hundred", Self::digit_to_word(self.hundreds))
    }
}

impl From<Period> for String {
    fn from(value: Period) -> Self {
        format!("{}{}", value.places_to_word(), value.name_to_word())
    }
}
