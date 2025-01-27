pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<&str> = Vec::new();
    for s in string.matches(&['{', '}', '(', ')', '[', ']']) {
        if ["{", "(", "["].contains(&s) {
            stack.push(s);
            continue;
        }
        match stack.pop() {
            Some(opening) => {
                if !matching_closing(opening, s) {
                    return false;
                }
            }
            None => return false,
        }
    }

    stack.is_empty()
}

fn matching_closing(open: &str, closing: &str) -> bool {
    match open {
        "{" => "}" == closing,
        "(" => ")" == closing,
        "[" => "]" == closing,
        _ => false,
    }
}
