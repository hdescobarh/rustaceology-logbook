use regex_lite::Regex;
const RULES: [&str; 4] = [
    r"^(?i)(?<prefix>[aeiou]|xr|yt)(?<suffix>\w*)(?<spaces>\s*)",
    r"^(?i)(?<suffix>[^aeiou]*qu)(?<prefix>\w*)(?<spaces>\s*)",
    r"^(?i)(?<suffix>[^aeiou]+)(?<prefix>[aeiou]\w*)(?<spaces>\s*)",
    r"^(?i)(?<suffix>[^aeiou]+)(?<prefix>y\w*)(?<spaces>\s*)",
];
pub fn translate(input: &str) -> String {
    input
        .split_inclusive(|c: char| c.is_ascii_whitespace())
        .map(|w| translate_word(w).unwrap_or_default())
        .collect()
}

fn translate_word(word: &str) -> Option<String> {
    for rule in RULES {
        if let Some(caps) = Regex::new(rule).ok()?.captures(word) {
            return Some([&caps["prefix"], &caps["suffix"], "ay", &caps["spaces"]].join(""));
        };
    }
    None
}
