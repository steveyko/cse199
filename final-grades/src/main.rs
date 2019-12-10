//!
//! This prints out final grades.
//!

mod field;
mod grade;

use field::Field;
use grade::Grade;
use preprocessing::clean_up;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::{self, Stdin};

type Record = HashMap<String, String>;

const NON_TOPHAT_TOTAL: f32 = 66.0;

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
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 3,
        "Please provide a TopHat total and bonus points."
    );
    let max_points = &args[1].parse::<f32>().unwrap() + NON_TOPHAT_TOTAL;
    let bonus_points = &args[2].parse::<f32>().unwrap();

    let mut rdr = clean_up_headers(csv::Reader::from_reader(io::stdin()))?;

    for result in rdr.deserialize() {
        let record: Record = result?;

        assert_point_range(&record);

        let last_name = record.get(Field::LastName.as_str()).expect("No last name");
        let first_name = record
            .get(Field::FirstName.as_str())
            .expect("No first name");

        let total = record
            .get(Field::Total.as_str())
            .expect("Total doesn't exist.")
            .parse::<f32>()
            .unwrap_or(0.0);
        assert!(
            total <= max_points,
            "{} is more than max ({}) for {}, {}.",
            total,
            max_points,
            last_name,
            first_name
        );
        let total = ((total + bonus_points) / max_points * 100.0)
            .ceil()
            .min(100.0);

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
            last_name,
            first_name,
            total,
            attendance
        );
    }
    Ok(())
}
