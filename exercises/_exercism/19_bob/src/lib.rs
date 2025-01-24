pub fn reply(message: &str) -> &str {
    let message: Vec<char> = message.trim().chars().collect();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let is_asking = message.last().is_some_and(|c| *c == '?');
    let mut only_alphabetic = message.iter().filter(|c| c.is_alphabetic()).peekable();
    let is_yelling = match only_alphabetic.peek() {
        Some(_) => only_alphabetic.all(|c| c.is_uppercase()),
        None => false,
    };
    match (is_asking, is_yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
