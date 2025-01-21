pub fn verse(n: u32) -> String {
    // [0-9]+ bottle[s]? of beer on the wall, [0-9]+ bottle[s]? of beer.\n
    // (Take (one|it) down and pass it around|Go to the store and buy some more),\s
    // ([0-9]+|no more) bottle[s]? of beer on the wall.
    let m = n.checked_sub(1);
    format!(
        "{before1} bottle{a} of beer on the wall, {before2} bottle{a} of beer.\n\
        {phrase}, {after} bottle{b} of beer on the wall.",
        before1 = how_many(Some(n), true),
        before2 = how_many(Some(n), false),
        a = use_plural(Some(n)),
        phrase = take_or_buy(m),
        after = how_many(m, false),
        b = use_plural(m)
    )
}

pub fn sing(start: u32, end: u32) -> String {
    todo!("sing verses {start} to {end}, inclusive")
}

fn how_many(n: Option<u32>, capitalize: bool) -> String {
    match n {
        Some(val) => {
            if val > 0 {
                val.to_string()
            } else {
                format!("{}o more", if capitalize { "N" } else { "n" })
            }
        }
        None => "99".to_string(),
    }
}

fn use_plural(n: Option<u32>) -> &'static str {
    match n {
        Some(value) => {
            if value == 1 {
                ""
            } else {
                "s"
            }
        }
        None => "s",
    }
}

fn take_or_buy(n: Option<u32>) -> String {
    match n {
        Some(value) => format!(
            "Take {} down and pass it around",
            if value == 0 { "it" } else { "one" }
        ),
        None => "Go to the store and buy some more".to_string(),
    }
}
