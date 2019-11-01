use std::io::{self, BufReader, BufRead};

fn main() {
    let mut bf = BufReader::new(io::stdin());

    let mut input = String::new();
    let _n = bf.read_line(&mut input);
    
    while let Some(n) = input.find('[') {
        if let Some(m) = input.find(']') {
            // -1 for a whitespace, +2 for ']' and a whitespace
            input.drain((n - 1)..(m + 2));
        }
    }
    while let Some(n) = input.find('|') {
        // +8 for seven digits
        input.drain(n..(n+8));
    }
    input = input.trim().to_string();
    println!("{}", input);

    for line in bf.lines() {
        if let Ok(input) = line {
            println!("{}", input);
        } 
    }
}
