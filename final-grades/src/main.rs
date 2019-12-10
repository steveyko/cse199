//!
//! This prints out final grades.
//!

mod constants;
mod field;
mod grade;
mod student;

use constants::NON_TOPHAT_TOTAL;
use preprocessing::clean_up;
use std::env;
use std::error::Error;
use std::io::{self, Stdin};
use student::Record;
use student::Student;

fn clean_up_headers(mut rdr: csv::Reader<Stdin>) -> Result<csv::Reader<Stdin>, Box<dyn Error>> {
    let mut v: Vec<String> = Vec::new();

    for field in rdr.headers()?.iter() {
        v.push(clean_up(field.to_string()));
    }

    rdr.set_headers(csv::StringRecord::from(v));

    Ok(rdr)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    assert!(
        args.len() == 3,
        "Please provide a TopHat total and bonus points."
    );

    let max_points = args[1].parse::<f32>().unwrap() + NON_TOPHAT_TOTAL;
    let bonus_points = args[2].parse::<f32>().unwrap();

    println!("{} {}", max_points, bonus_points);

    let mut rdr = clean_up_headers(csv::Reader::from_reader(io::stdin()))?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        let student = Student::new(&record, max_points, bonus_points);

        println!("{}", student.to_string());
    }
    Ok(())
}
