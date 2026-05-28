use std::fs;
use crate::models::Student;
use std::error::Error;

pub fn read_students(path: &str) -> Result<Vec<Student>, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let students: Vec<Student> = serde_json::from_str(&data)?;
    Ok(students)
}

pub fn write_students(path: &str, students: &Vec<Student>) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(students)?;
    fs::write(path, json)?;
    Ok(())
}
