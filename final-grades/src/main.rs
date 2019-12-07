//!
//! This prints out final grades.
//!

use std::collections::HashMap;
use std::error::Error;
use std::io;

const LAST_NAME: &str = "Last Name";
const FIRST_NAME: &str = "First Name";
const TOTAL: &str = "Total";
const ATTENDANCE: &str = "TH-Attendance";
const ID: &str = "Student ID";

const ATTENDANCE_TOTAL: usize = 26; // total attendance for the semester

enum Grade {
    A,
    AMinus,
    BPlus,
    B,
    BMinus,
    CPlus,
    C,
    CMinus,
    DPlus,
    D,
    F,
}

impl Grade {
    fn new(total: f32, attendance: usize) -> Grade {
        assert!(attendance <= ATTENDANCE_TOTAL);

        let mut grade: Grade = match total {
            x if x < 50.0 => Grade::F,
            x if x < 58.0 => Grade::D,
            x if x < 62.0 => Grade::DPlus,
            x if x < 66.0 => Grade::CMinus,
            x if x < 72.0 => Grade::C,
            x if x < 76.0 => Grade::CPlus,
            x if x < 80.0 => Grade::BMinus,
            x if x < 84.0 => Grade::B,
            x if x < 88.0 => Grade::BPlus,
            x if x < 92.0 => Grade::AMinus,
            _ => Grade::A,
        };

        let downgrade_by: usize = (ATTENDANCE_TOTAL - attendance) / 3;

        for _ in 0..downgrade_by {
            grade.downgrade();
        }

        grade
    }

    fn downgrade(&mut self) {
        *self = match self {
            Grade::A => Grade::AMinus,
            Grade::AMinus => Grade::BPlus,
            Grade::BPlus => Grade::B,
            Grade::B => Grade::BMinus,
            Grade::BMinus => Grade::CPlus,
            Grade::CPlus => Grade::C,
            Grade::C => Grade::CMinus,
            Grade::CMinus => Grade::DPlus,
            Grade::DPlus => Grade::D,
            Grade::D => Grade::F,
            Grade::F => Grade::F,
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Grade::A => "A",
            Grade::AMinus => "A-",
            Grade::BPlus => "B+",
            Grade::B => "B",
            Grade::BMinus => "B-",
            Grade::CPlus => "C+",
            Grade::C => "C",
            Grade::CMinus => "C-",
            Grade::DPlus => "D+",
            Grade::D => "D",
            Grade::F => "F",
        }
    }
}

type Record = HashMap<String, String>;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

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
