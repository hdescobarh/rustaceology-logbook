pub fn encode(source: &str) -> String {
    let mut output = String::new();
    let mut buffer: Option<(char, usize)> = None;
    for current in source.chars() {
        match buffer {
            Some((letter, times)) if current == letter => buffer = Some((letter, times + 1)),
            Some((letter, times)) => {
                encode_letter(letter, times, &mut output);
                buffer = Some((current, 1))
            }
            None => buffer = Some((current, 1)),
        }
    }

    if let Some((letter, times)) = buffer {
        encode_letter(letter, times, &mut output);
    }
    output
}

fn encode_letter(letter: char, times: usize, output: &mut String) {
    let encoded_letter = if times > 1 {
        format!("{times}{letter}")
    } else {
        format!("{letter}")
    };
    output.push_str(&encoded_letter);
}

pub fn decode(source: &str) -> String {
    source
        .split_inclusive(|c: char| c.is_ascii_alphabetic() || c == ' ')
        .map(|s| {
            let (times, letter) = s.split_at(s.len() - 1);
            let number_repeats = if times.is_empty() {
                1
            } else {
                times.parse::<usize>().unwrap()
            };
            letter.repeat(number_repeats)
        })
        .collect()
}
