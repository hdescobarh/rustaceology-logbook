pub fn actions(n: u8) -> Vec<&'static str> {
    let mut output = Vec::with_capacity(4);
    for step in 0..5 {
        match ((n >> step) & 1, step) {
            (1, 0) => output.push("wink"),
            (1, 1) => output.push("double blink"),
            (1, 2) => output.push("close your eyes"),
            (1, 3) => output.push("jump"),
            (1, 4) => output.reverse(),
            _ => continue,
        };
    }
    output
}
