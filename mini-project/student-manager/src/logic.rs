use crate::models::Student;

pub fn filter_top_students(students: Vec<Student>) -> Vec<Student> {
    students
        .into_iter()
        .filter(|s| s.score >= 80.0)
        .collect()
}
