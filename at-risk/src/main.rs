//!
//! This prints out at-risk students according to the thresholds set below.
//!

use std::collections::HashMap;
use std::error::Error;
use std::io;

const LAST_NAME: &str = "Last Name";
const FIRST_NAME: &str = "First Name";
const TOTAL: &str = "Total";
const ATTENDANCE: &str = "TH-Attendance";
//const TOTAL_THRESHOLD: f32 = 10.0; // 10 or less
//const ATTENDANCE_THRESHOLD: f32 = 5.0; // 5 or less (9 is the max)
const TOTAL_THRESHOLD: f32 = 50.0; // 10 or less
const ATTENDANCE_THRESHOLD: f32 = 9.0; // 5 or less (9 is the max)

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
            .unwrap_or(0.0);

        if (total > TOTAL_THRESHOLD) && (attendance > ATTENDANCE_THRESHOLD) {
            continue;
        }

        println!(
            "{:?}",
            record.get(LAST_NAME).expect("Last name doesn't exist")
        );
        println!(
            "{:?}",
            record.get(FIRST_NAME).expect("First name doesn't exist")
        );
        println!("{:?}", record.get(TOTAL).expect("Total doesn't exist"));
        println!(
            "{:?}",
            record.get(ATTENDANCE).expect("Attendance doesn't exist")
        );
    }
    Ok(())
}
