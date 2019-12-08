//!
//! This prints out final grades.
//!

mod grade;

use grade::Grade;
use preprocessing::clean_up;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Stdin};

const LAST_NAME: &str = "Last Name";
const FIRST_NAME: &str = "First Name";
const TOTAL: &str = "Total";
const ATTENDANCE: &str = "TH-Attendance";
const ID: &str = "Student ID";

type Record = HashMap<String, String>;

fn clean_up_headers(mut rdr: csv::Reader<Stdin>) -> Result<csv::Reader<Stdin>, Box<dyn Error>> {
    let mut v: Vec<String> = Vec::new();
    for field in rdr.headers()?.iter() {
        v.push(clean_up(field.to_string()));
    }
    rdr.set_headers(csv::StringRecord::from(v));

    Ok(rdr)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    rdr = clean_up_headers(rdr)?;

    for result in rdr.deserialize() {
        let record: Record = result?;

        let total = record
            .get(TOTAL)
            .expect("Total doesn't exist.")
            .parse::<f32>()
            .unwrap_or(0.0);
        let attendance = record
            .get(ATTENDANCE)
            .expect("Attendance doesn't exist.")
            .parse::<f32>()
            .unwrap_or(0.0)
            .ceil() as usize;

        let grade = Grade::new(total, attendance);

        println!(
            "{}, {}, {}, {}, {}, {}",
            record.get(ID).expect("ID doesn't exist"),
            grade.to_string(),
            record.get(LAST_NAME).expect("Last name doesn't exist"),
            record.get(FIRST_NAME).expect("First name doesn't exist"),
            total,
            attendance
        );
    }
    Ok(())
}
