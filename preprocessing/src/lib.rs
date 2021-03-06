//! This cleans up the CSV header of a UBlearns CSV file.
//!
//! For example, "Total [Total Pts: up to 191.7 Score] |1263507" becomes "Total"
//!

pub fn clean_up(mut input: String) -> String {
    while let Some(n) = input.find('[') {
        if let Some(m) = input.find(']') {
            // -1 for a whitespace, +2 for ']' and a whitespace
            input.drain((n - 1)..(m + 2));
        }
    }

    while let Some(n) = input.find('|') {
        // +8 for seven digits
        input.drain(n..(n + 8));
    }
    input.trim().to_string()
}
