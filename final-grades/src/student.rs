//!
//! This defines struct Student.
//!

use crate::field::Field;
use crate::grade::Grade;
use log::debug;
use std::collections::HashMap;

pub type Record = HashMap<String, String>;

#[derive(Debug)]
pub struct Student<'a> {
    student_id: &'a str,
    grade: &'a str,
    last_name: &'a str,
    first_name: &'a str,
    total: f32,
    attendance: usize,
}

impl Student<'_> {
    pub fn new(record: &Record, max_points: f32, bonus_points: f32) -> Student {
        Student::assert_point_range(&record);

        let student_id = record
            .get(Field::StudentID.as_str())
            .expect("ID doesn't exist");

        let last_name = record.get(Field::LastName.as_str()).expect("No last name");

        let first_name = record
            .get(Field::FirstName.as_str())
            .expect("No first name");

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

        let grade = Grade::new(total, attendance).as_str();

        Student {
            student_id,
            last_name,
            first_name,
            total,
            attendance,
            grade,
        }
    }

    pub fn to_string(&self) -> String {
        debug!("Student: {:#?}", self);
        format!("{},{}", self.student_id, self.grade)
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
}
