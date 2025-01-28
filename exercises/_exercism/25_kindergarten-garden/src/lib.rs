const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_id = STUDENTS.iter().position(|s| s == &student).unwrap();
    diagram
        .split_ascii_whitespace()
        .flat_map(|r| r.chars().skip(student_id * 2).take(2))
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
