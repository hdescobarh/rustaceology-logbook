use std::iter::repeat;

pub fn encode(source: &str) -> String {
    // source contains only alphabetic ASCII
    let mut output = String::new();
    let mut previous = None::<char>;
    let mut count = 0;

    for current in source.chars() {
        match previous {
            Some(v) if v == current => count += 1,
            Some(v) => {
                let encoded = if count > 1 {
                    format!("{count}{v}")
                } else {
                    format!("{v}")
                };
                output.push_str(&encoded);
                previous = Some(current);
                count = 1;
            }
            None => {
                previous = Some(current);
                count += 1;
            }
        };
    }
    if let Some(v) = previous {
        let encoded = if count > 1 {
            format!("{count}{v}")
        } else {
            format!("{v}")
        };
        output.push_str(&encoded);
    }
    output
}

pub fn decode(source: &str) -> String {
    todo!("Return the run-length decoding of {source}.");
}
