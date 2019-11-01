//!
//! This prints out at-risk students according to the thresholds set below.
//!

use std::error::Error;
use std::io;

const LAST_NAME: &str = "Last Name";
const FIRST_NAME: &str = "First Name";
const TOTAL: &str = "Total";
const ATTENDANCE: &str = "TH-Attendance";
const TOTAL_THRESHOLD: f32 = 10.0; // 10 or less
const ATTENDANCE_THRESHOLD: f32 = 5.0; // 5 or less (9 is the max)

struct HeaderIndex {
    last_name: usize,
    first_name: usize,
    total: usize,
    attendance: usize,
}

impl HeaderIndex {
    fn new(headers: csv::StringRecord) -> HeaderIndex {
        let mut last_name = None;
        let mut first_name = None;
        let mut total = None;
        let mut attendance = None;
        let mut i = 0;

        for field in headers.iter() {
            match field {
                LAST_NAME => last_name = Some(i),
                FIRST_NAME => first_name = Some(i),
                TOTAL => total = Some(i),
                ATTENDANCE => attendance = Some(i),
                _ => (),
            }
            i += 1;
        }

        let last_name = last_name.expect("Missing last name field");
        let first_name = first_name.expect("Missing first name field");
        let total = total.expect("Missing total field");
        let attendance = attendance.expect("Missing attendance field");

        HeaderIndex {
            last_name,
            first_name,
            total,
            attendance,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let index = HeaderIndex::new(rdr.headers()?.clone());

    for result in rdr.records() {
        let record = result?;

        let total = record
            .get(index.total)
            .expect("Total doesn't exist.")
            .parse::<f32>()
            .unwrap_or(0.0);
        let attendance = record
            .get(index.attendance)
            .expect("Attendance doesn't exist.")
            .parse::<f32>()
            .unwrap_or(0.0);

        if (total > TOTAL_THRESHOLD) && (attendance > ATTENDANCE_THRESHOLD) {
            continue;
        }

        println!(
            "{:?}",
            record.get(index.last_name).expect("Last name doesn't exist")
        );
        println!(
            "{:?}",
            record.get(index.first_name).expect("First name doesn't exist")
        );
        println!(
            "{:?}",
            record.get(index.total).expect("Total doesn't exist")
        );
        println!(
            "{:?}",
            record.get(index.attendance).expect("Attendance doesn't exist")
        );
    }
    Ok(())
}
