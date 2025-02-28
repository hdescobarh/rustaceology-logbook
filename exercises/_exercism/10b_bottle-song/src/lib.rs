use std::fmt::Display;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map_while(|i| start_bottles.checked_sub(i).and_then(Verse::new))
        .map(|v| v.to_string())
        .collect()
}

struct Verse(u32);

impl Verse {
    pub fn new(bottles: u32) -> Option<Self> {
        if bottles == 0 || bottles > 10 {
            return None;
        }
        Some(Self(bottles))
    }
}

impl Display for Verse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (current, remaining) = match (self.0, self.0 - 1) {
            (1, 0) => ("One", "no"),
            (2, 1) => ("Two", "one"),
            (3, 2) => ("Three", "two"),
            (4, 3) => ("Four", "three"),
            (5, 4) => ("Five", "four"),
            (6, 5) => ("Six", "five"),
            (7, 6) => ("Seven", "six"),
            (8, 7) => ("Eight", "seven"),
            (9, 8) => ("Nine", "eight"),
            (10, 9) => ("Ten", "nine"),
            _ => return Ok(()),
        };

        let (ss1, ss2) = match self.0 {
            1 => ("", "s"),
            2 => ("s", ""),
            _ => ("s", "s"),
        };

        writeln!(
            f,
            "{current} green bottle{ss1} hanging on the wall,\n\
            {current} green bottle{ss1} hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be {remaining} green bottle{ss2} hanging on the wall.\n",
        )
    }
}
