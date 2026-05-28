mod models;
mod io;
mod logic;

use io::{read_students, write_students};
use logic::filter_top_students;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let students = read_students("data/students.json")?;

    println!("Loaded {} students", students.len());

    let top_students = filter_top_students(students);

    write_students("data/result.json", &top_students)?;

    println!("Saved filtered students to result.json");

    Ok(())
}
