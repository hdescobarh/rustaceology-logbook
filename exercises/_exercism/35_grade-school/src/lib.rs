use std::collections::BTreeMap;

#[derive(Clone)]
struct StudentsList {
    grade: u32,
    students: Vec<String>, // always sorted
}

impl StudentsList {
    pub fn new(grade: u32) -> Self {
        Self {
            grade,
            students: Vec::new(),
        }
    }

    pub fn with_initial_student(mut self, student: &str) -> Self {
        if self.students.is_empty() {
            self.students.push(student.to_string())
        }
        self
    }

    pub fn add(&mut self, student: &str) -> Result<(), String> {
        let student = student.to_string();
        if self.students.is_empty() {
            self.students.push(student);
            return Ok(());
        }
        match self.students.binary_search(&student) {
            Ok(_) => {
                return Err(format!(
                    "Student {student} already is in grade {grade}",
                    grade = self.grade
                ))
            }
            Err(index) => self.students.insert(index, student),
        };
        Ok(())
    }

    pub fn contains(&self, student: &str) -> bool {
        self.students.binary_search(&student.to_string()).is_ok()
    }
}

impl From<StudentsList> for Vec<String> {
    fn from(value: StudentsList) -> Self {
        value.students
    }
}

pub struct School {
    roster: BTreeMap<u32, StudentsList>,
}

impl School {
    pub fn new() -> School {
        Self {
            roster: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) -> Result<(), String> {
        for (grade, list) in self.roster.iter() {
            if list.contains(student) {
                return Err(format!("Student {student} already is in grade {grade}"));
            }
        }

        match self.roster.get_mut(&grade) {
            Some(list) => list.add(student),
            None => {
                self.roster.insert(
                    grade,
                    StudentsList::new(grade).with_initial_student(student),
                );
                Ok(())
            }
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster
            .get(&grade)
            .map(|list| Vec::from(list.clone()))
            .unwrap_or_default()
    }
}
