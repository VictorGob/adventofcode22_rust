use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &str = "input_d3.txt";
    let mut count = 0;
    // Generate alphabet
    let raw_alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let mut alpha: HashMap<char, u32> = HashMap::new();
    for (i, ltr) in raw_alpha.enumerate() {
        alpha.insert(ltr, 1 + (i as u32));
    }
    
    match read_lines(filename) {
        Ok(lines) => lines
        .for_each(|line| 
            
        ),
        Err(p_err) => panic!("Error: {}", p_err),
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    Ok(io::BufReader::new(match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("Error while opening the file, error={}", error),
    })
    .lines())
}
