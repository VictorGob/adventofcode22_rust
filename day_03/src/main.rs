use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn main() {
    let filename: &str = "input_d3.txt";
    let mut count: u32 = 0;
    // Generate alphabet
    let raw_alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let mut alpha: HashMap<char, u32> = HashMap::new();
    for (i, ltr) in raw_alpha.enumerate() {
        alpha.insert(ltr, 1 + (i as u32));
    }

    match read_lines(filename) {
        Ok(lines) => lines.for_each(|line| match line {
            Ok(rucksack) => {
                let (split_a, split_b) = rucksack.split_at(rucksack.len() / 2);
                for c in split_a.chars() {
                    if split_b.contains(c) {
                        count += alpha.get(&c).unwrap();
                        break;
                    }
                }
            }
            Err(err) => panic!("Unable to parse! {}", err),
        }),
        Err(p_err) => panic!("Error: {}", p_err),
    }
    println!("Count: {}", count);

    /* Part 2 */
    // From https://fasterthanli.me/series/advent-of-code-2022/part-3#part-2
    let mut count: u32 = 0;
    let mut grouplines: Vec<&str> = Vec::new();
    match read_lines(filename) {
        Ok(lines) => lines.chunks(3).into_iter().for_each(|group| {
            for line in group {
                let no_rep = line.into_iter().unique();
                // println!("{:?}", line.unwrap());
                println!("{:?}", no_rep);
            }
        }),
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
