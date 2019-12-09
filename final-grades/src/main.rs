//!
//! This prints out final grades.
//!

mod field;
mod grade;

use field::Field;
use grade::Grade;
use preprocessing::clean_up;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Stdin};

type Record = HashMap<String, String>;

fn clean_up_headers(mut rdr: csv::Reader<Stdin>) -> Result<csv::Reader<Stdin>, Box<dyn Error>> {
    let mut v: Vec<String> = Vec::new();
    for field in rdr.headers()?.iter() {
        v.push(clean_up(field.to_string()));
    }
    rdr.set_headers(csv::StringRecord::from(v));

    Ok(rdr)
}

fn assert_point_range(record: &Record) {
    for (key, val) in record.iter() {
        if !Field::from_str(key).is_none() {
            continue;
        }

        let val = val.parse::<f32>().unwrap_or(0.0);
        assert!(val <= 3.0 && val >= 0.0, "Wrong range: {}, {}", key, val);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = clean_up_headers(csv::Reader::from_reader(io::stdin()))?;

    for result in rdr.deserialize() {
        let record: Record = result?;

        assert_point_range(&record);

        let total = record
            .get(Field::Total.as_str())
            .expect("Total doesn't exist.")
            .parse::<f32>()
            .unwrap_or(0.0);
        let attendance = record
            .get(Field::Attendance.as_str())
            .expect("Attendance doesn't exist.")
            .parse::<f32>()
            .unwrap_or(0.0)
            .ceil() as usize;

        let grade = Grade::new(total, attendance);

        println!(
            "{}, {}, {}, {}, {}, {}",
            record
                .get(Field::StudentID.as_str())
                .expect("ID doesn't exist"),
            grade.to_string(),
            record
                .get(Field::LastName.as_str())
                .expect("Last name doesn't exist"),
            record
                .get(Field::FirstName.as_str())
                .expect("First name doesn't exist"),
            total,
            attendance
        );
    }
    Ok(())
}
