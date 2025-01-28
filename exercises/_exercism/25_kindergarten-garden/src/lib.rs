pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_id = get_student_id(student).unwrap();
    diagram
        .split_ascii_whitespace()
        .take(2)
        .flat_map(|r| {
            r.chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .map(|a| [a[0], a[1]])
                .nth(student_id)
                .unwrap()
        })
        .map(|plant_code| translate_from_code(plant_code).unwrap())
        .collect()
}

fn translate_from_code(code: char) -> Option<&'static str> {
    match code {
        'G' => Some("grass"),
        'C' => Some("clover"),
        'R' => Some("radishes"),
        'V' => Some("violets"),
        _ => None,
    }
}

fn get_student_id(student: &str) -> Option<usize> {
    [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ]
    .iter()
    .position(|s| s == &student)
}
