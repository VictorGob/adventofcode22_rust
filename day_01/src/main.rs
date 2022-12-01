use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "input.txt";
    let mut cal_count: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        let mut count: u32 = 0;
        for line in lines {
            if let Ok(cal) = line {
                if cal.is_empty(){
                    cal_count.push(count);
                    count = 0;
                } else {
                    let cal_u32 = cal.trim_end().parse::<u32>().expect("Unable to parse value to u32"); 
                    count = count + cal_u32;
                }
            } 
        }
        // When lines ends, add last count
        cal_count.push(count);
    } 
    // Sort and reverse vector
    cal_count.sort_by(|a, b| b.cmp(a));
    println!("Elf carrying the most Calories: {}", cal_count[0]);
    let top3_cal: u32 = (cal_count[0..3]).to_vec().iter().sum();
    println!("How many Calories are those Elves carrying in total?: {}", top3_cal);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}