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
        .map(translate_word)
        .collect()
}

fn translate_word(word: &str) -> String {
    for rule in &RULES[..RULES.len()] {
        let re = Regex::new(rule).unwrap();
        if let Some(caps) = re.captures(word) {
            return [&caps["prefix"], &caps["suffix"], "ay", &caps["spaces"]].join("");
        };
    }
    let re = Regex::new(RULES.last().unwrap()).unwrap();
    re.replace_all(word, "$prefix${suffix}ay").to_string()
}
