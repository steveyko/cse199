//! This cleans up the CSV header of a UBlearns CSV file.
//!
//! For example, "Total [Total Pts: up to 191.7 Score] |1263507" becomes "Total"
//!

use preprocessing::clean_up;
use std::io::{self, BufRead, BufReader};

fn main() {
    let mut bf = BufReader::new(io::stdin());

    let mut input = String::new();
    let _n = bf.read_line(&mut input);

    println!("{}", clean_up(input));

    for line in bf.lines() {
        if let Ok(input) = line {
            println!("{}", input);
        }
    }
}
